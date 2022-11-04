// imports
use anyhow::Result as AResult;
use chrono::prelude::*;
use secp256k1::{ecdsa::Signature as TxnSignature, Message, PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// local
use super::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug)]
pub enum TxnType {
    Transfer = 1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxnBody {
    pub amt: u128,
    pub recv_pubkey: PublicKey,
    pub sender_pubkey: PublicKey,
}

// TODO: turn this into a full-fledged wrapper around txn data
#[derive(Serialize, Deserialize, Debug)]
pub struct Txn {
    pub body: TxnBody,
    /// Ecdsa
    pub signature: Option<TxnSignature>,
    /// The time the txn was created
    pub system_time: u64,
    pub txn_type: TxnType,
    pub id: String,
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
        let id = Uuid::new_v4().to_string();

        // TODO: make `new` assoc fxn
        let body = TxnBody {
            sender_pubkey,
            recv_pubkey,
            amt,
        };
        Self {
            system_time, // timestamp
            body,
            signature: Option::None,
            txn_type,
            id,
        }
    }

    /// Get the hash digest of the transaction message
    pub fn get_txn_msg(&self) -> Message {
        let txn_msg_bytes = self.message_bytes();

        // get hash digest of txn
        use blake3::traits::digest::Digest;
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"message-v0");
        hasher.update(&txn_msg_bytes);
        let hash = hasher.finalize().to_vec();

        // return the message digest
        Message::from_slice(&hash).expect("trying to get txn message")
    }

    /// Create and return a message signature based on
    /// the contents of the transaction
    pub fn get_msg_signature(&self, wallet: &Wallet) -> TxnSignature {
        let msg = self.get_txn_msg();

        let sig: TxnSignature = wallet.get_msg_signature(&msg);

        sig
    }

    /// Check if the existing signature matches the transaction object
    /// Only works if signature is set
    /// Signature is a ecdsa signature
    pub fn validate_signature(&self, sig: TxnSignature, wallet: Wallet) -> Option<bool> {
        let msg = self.get_txn_msg();
        let pbkey = wallet.get_pubkey();

        // init
        let secp = Secp256k1::verification_only();

        Some(secp.verify_ecdsa(&msg, &sig, &pbkey).is_ok());
        if let Some(sig) = self.signature {
            // return bool
            Some(true)
        } else {
            None
        }
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
}
