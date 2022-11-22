// imports
use anyhow::Result as AResult;
use chrono::prelude::*;
use secp256k1::{ecdsa::Signature as TxnSignature, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// local
use super::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TxnType {
    Transfer = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TxnBody {
    pub amt: u128,
    pub recv_pubkey: PublicKey,
    pub sender_pubkey: PublicKey,
}

// TODO: turn this into a full-fledged wrapper around txn data
#[derive(Debug, Clone, Copy)]
pub struct Txn {
    pub body: TxnBody,
    /// Ecdsa
    pub signature: Option<TxnSignature>,
    /// The time the txn was created
    pub system_time: u64,
    pub txn_type: TxnType,
    pub hash: Option<Message>,
}
impl Txn {
    /// Transaction constructor fxn
    ///
    /// creates a transaction `object`
    /// is public
    pub fn new(
        sender_pubkey: PublicKey,
        recv_pubkey: PublicKey,
        amt: u128,
        txn_type: TxnType,
    ) -> Self {
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();

        // TODO: make `new` assoc fxn
        let body = TxnBody {
            sender_pubkey,
            recv_pubkey,
            amt,
        };
        let hash = Self::get_txn_hash(&body);
        Self {
            system_time, // timestamp
            body,
            signature: Option::None,
            txn_type,
            hash: Some(hash),
        }
    }

    /// Signed transaction constructor fxn
    ///
    /// Create a transaction and sign it with given wallet
    pub fn new_signed(
        wallet: Wallet,
        recv_pbkey: PublicKey,
        amt_to_send: u128,
        txn_type: TxnType,
    ) -> Txn {
        let mut txn = Self::new(wallet.get_pbkey(), recv_pbkey, amt_to_send, txn_type);
        let sig = wallet.sign(&mut txn);

        let is_signed = txn.signature.unwrap() == sig;

        match is_signed {
            true => return txn,
            false => panic!("transaction is not signed"),
        }
    }
    /// Hash the transaction and store the output on the Txn object
    pub fn hash(&mut self) -> Message {
        let hash = self.get_txn_msg();

        self.hash = Some(hash);

        hash
    }
    /// Get the hash digest of the transaction message - associated fxn
    pub fn get_txn_hash(txn_body: &TxnBody) -> Message {
        let txn_msg_bytes = Self::get_msg_bytes(txn_body);

        // get hash digest of txn
        use blake3::traits::digest::Digest;
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"message-v0");
        hasher.update(&txn_msg_bytes);
        let hash = hasher.finalize().to_vec();

        // return the message digest
        Message::from_slice(&hash).expect("trying to get txn message")
    }
    /// Get the hash digest of the transaction message - method
    pub fn get_txn_msg(&self) -> Message {
        Self::get_txn_hash(&self.body)
    }

    /// Create and return a message signature based on
    /// the contents of the transaction
    pub fn get_msg_signature(&self, wallet: &Wallet) -> TxnSignature {
        let msg = self.get_txn_msg();

        let sig: TxnSignature = wallet.get_msg_signature(&msg);

        sig
    }

    /// Wrapper for wallet.sign()
    ///
    /// 1) Sign the transaction
    /// 2) Add signature to transaction body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> AResult<()> {
        wallet.sign(self);

        Ok(())
    }
    /// Get the message in bytes from transaction body
    pub fn get_msg_bytes(txn_body: &TxnBody) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(txn_body).unwrap();

        encoded
    }
    /// Get the message in bytes
    pub fn message_bytes(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serde_json::to_vec(&self.body).unwrap();

        encoded
    }
}

mod tests {
    use secp256k1::KeyPair;
    use std::{fs::File, io::BufReader};
    fn init_test_vars() -> (KeyPair, KeyPair) {
        let sender_kp = create_keypair_from_file("test_key.json".to_string());
        let recv_kp = create_keypair_from_file("test_key_recv.json".to_string());

        (sender_kp, recv_kp)
    }
    fn create_keypair_from_file(filepath: String) -> KeyPair {
        let f = File::open(filepath).unwrap();
        let reader = BufReader::new(f);
        let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();
        let secp = Secp256k1::new();

        let keypair = KeyPair::from_seckey_slice(&secp, &key_json).unwrap();

        keypair
    }
    fn create_transfer_txn_msg() -> Message {
        let (sender_kp, recv_kp) = init_test_vars();

        // turn the raw txn into message
        let txn = Txn::new(
            sender_kp.public_key(),
            recv_kp.public_key(),
            100,
            TxnType::Transfer,
        );

        txn.get_txn_msg()
    }
    use crate::ledger::txn;

    use super::*;

    #[test]
    fn create_unsigned_txn() {
        let answer = "dbb52e564948611791557341a83560a75e8f375490433e319b8dbef6d78e1a9f";
        let msg = create_transfer_txn_msg();

        assert!(msg.to_string() == answer.to_string(), "{msg:?}");
    }
    #[test]
    fn create_signed_txn() {
        let (sender_kp, _recv_kp) = init_test_vars();
        let msg = create_transfer_txn_msg();

        // get signature
        let secp = Secp256k1::new();
        let signature: TxnSignature = secp.sign_ecdsa(&msg, &sender_kp.secret_key());
        let answer = "3045022100fd15a048c36c5b3805da858e7a8a68d6c4bfd45450b0bf4bdfefcb7e92b553f30220445729607f6a2fb18c592922bd2bc44c0daf66fa43205820c9073c3b3568654f";

        assert!(signature.to_string() == answer.to_string(), "{signature:?}",);
    }
    #[test]
    fn create_new_signed_txn() {
        let (kp_send, kp_recv) = init_test_vars();
        let wallet_send = Wallet::new_from_kp(&kp_send);
        let signed_txn = Txn::new_signed(wallet_send, kp_recv.public_key(), 100, TxnType::Transfer);
        let msg = signed_txn.get_txn_msg();

        // get signature
        let secp = Secp256k1::new();
        let signature: TxnSignature = secp.sign_ecdsa(&msg, &kp_send.secret_key());
        let answer = "3045022100fd15a048c36c5b3805da858e7a8a68d6c4bfd45450b0bf4bdfefcb7e92b553f30220445729607f6a2fb18c592922bd2bc44c0daf66fa43205820c9073c3b3568654f";

        assert!(signature.to_string() == answer.to_string(), "{signature:?}",);
    }
}
