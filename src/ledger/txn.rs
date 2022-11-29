// imports
// use anyhow::Result as AResult;
use chrono::prelude::*;
use secp256k1::{PublicKey, Secp256k1};
use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};
// local
use crate::ledger::wallet::Wallet;
// exported types
pub type TxnSig = [u8; 64];
pub type TxnHash = [u8; 32];
pub use blake3::Hash as BlakeHash;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TxnType {
    Transfer = 1,
}

/// Serializable body of the transaction
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Txn {
    pub amt: u128,
    pub pbkey_send: PublicKey,
    pub pbkey_recv: PublicKey,
    // The time the txn was created
    pub system_time: u64,
    /// Type of transaction - as int
    pub txn_type: TxnType,
    /// Blake3 hash as byte array
    pub hash: TxnHash,
    /// Ecdsa signature as byte array
    #[serde(with = "BigArray")]
    pub signature: TxnSig,
}

impl Txn {
    /// Transaction constructor fxn
    ///
    /// creates a transaction `object`
    /// is public
    pub fn new(
        pbkey_send: PublicKey,
        pbkey_recv: PublicKey,
        // amt to send
        amt: u128,
        // type of transaction
        txn_type: TxnType,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();

        let mut txn = Self {
            pbkey_send,
            pbkey_recv,
            amt,
            system_time,
            txn_type,
            hash: [0u8; 32],
            signature: [0u8; 64],
        };

        // set the hash with the body
        txn.set_hash();
        // return the txn
        txn
    }

    /// Create and return a new signed transaction.
    ///
    /// Receives `Wallet` instance for signing.
    /// Uses `Txn::new()` assoc fxn. to construct the txn, and signs the txn with given wallet.
    pub fn new_signed(
        wallet: Wallet,
        pbkey_recv: PublicKey,
        amt_to_send: u128,
        txn_type: TxnType,
    ) -> Txn {
        let pbkey_send = wallet.get_pbkey();
        let mut txn = Self::new(pbkey_send, pbkey_recv, amt_to_send, txn_type);

        txn.sign(&wallet);

        txn
    }
    /// Compute the hash digest of the transaction message - associated fxn
    pub fn get_hash(txn: &Txn) -> TxnHash {
        let mut adj_txn_body = txn.clone();
        // set blank vars
        adj_txn_body.hash = [0u8; 32];
        adj_txn_body.signature = [0u8; 64];

        // serialize to a byte vector
        let txn_msg_bytes: Vec<u8> = serde_json::to_vec(txn).unwrap();

        // get hash digest of txn
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"message-v0");
        hasher.update(&txn_msg_bytes);
        let hash: TxnHash = hasher.finalize().as_bytes().to_owned();

        hash
    }
    /// Method wrapper/analog for `get_hash()`
    pub fn hash(&self) -> TxnHash {
        Self::get_hash(&self)
    }
    /// Get hash for txn and set on txn object and store the output on the Txn object
    ///
    /// Returns hash
    pub fn set_hash(&mut self) -> TxnHash {
        let hash = self.hash();
        self.hash = hash;

        hash
    }
    /// Get hash with traits from Blake3 library
    ///
    /// Returns hash
    pub fn get_blake_hash(&self) -> BlakeHash {
        let txn_hash = self.hash();
        BlakeHash::from(txn_hash)
    }
    /// Create and return a message signature based on
    ///    the contents of the transaction
    pub fn get_signature(&self, wallet: &Wallet) -> TxnSig {
        let msg: TxnHash = self.hash();

        let sig = wallet.get_signature(&msg);

        sig
    }
    pub fn set_signature(&mut self, signature: TxnSig) {
        self.signature = signature;
    }
    /// Add the signature to the transaction body in place.
    ///
    /// 1) Sign the transaction
    /// 2) Add signature to transaction body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> TxnSig {
        let sig: TxnSig = self.get_signature(wallet);

        // 2) set signature - add sig to txn body
        self.set_signature(sig.clone());

        // 3) return signature
        sig
    }
}

mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::ledger::general::SecpEcdsaSignature;
    #[allow(unused_imports)]
    use secp256k1::{KeyPair, Message};
    use std::{fs::File, io::BufReader};
    #[allow(dead_code)]
    fn init_test_vars() -> (KeyPair, KeyPair) {
        let sender_kp = create_keypair_from_file("test_key.json".to_string());
        let recv_kp = create_keypair_from_file("test_key_recv.json".to_string());

        (sender_kp, recv_kp)
    }
    #[allow(dead_code)]
    fn create_keypair_from_file(filepath: String) -> KeyPair {
        let f = File::open(filepath).unwrap();
        let reader = BufReader::new(f);
        let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();
        let secp = Secp256k1::new();

        let keypair = KeyPair::from_seckey_slice(&secp, &key_json).unwrap();

        keypair
    }
    #[allow(dead_code)]
    fn create_transfer_txn_msg() -> TxnHash {
        let (sender_kp, recv_kp) = init_test_vars();

        // turn the raw txn into message
        let txn = Txn::new(
            sender_kp.public_key(),
            recv_kp.public_key(),
            100,
            TxnType::Transfer,
        );

        txn.hash()
    }

    #[test]
    fn create_unsigned_txn() {
        let answer = "dbb52e564948611791557341a83560a75e8f375490433e319b8dbef6d78e1a9f";
        let msg = create_transfer_txn_msg();
        let msg = msg.as_slice();
        let msg = String::from_utf8(msg.to_ascii_lowercase()).unwrap();

        assert!(msg == answer, "{:?}", msg);
    }
    #[test]
    fn create_signed_txn() {
        let (sender_kp, _recv_kp) = init_test_vars();
        let msg = create_transfer_txn_msg();

        // get signature
        let secp = Secp256k1::new();
        let signature: SecpEcdsaSignature =
            secp.sign_ecdsa(&Message::from_slice(&msg).unwrap(), &sender_kp.secret_key());
        let answer = "3045022100fd15a048c36c5b3805da858e7a8a68d6c4bfd45450b0bf4bdfefcb7e92b553f30220445729607f6a2fb18c592922bd2bc44c0daf66fa43205820c9073c3b3568654f";

        assert!(signature.to_string() == answer.to_string(), "{signature:?}",);
    }
    #[test]
    fn create_new_signed_txn() {
        let (kp_send, kp_recv) = init_test_vars();
        let wallet_send = Wallet::new_from_kp(&kp_send);
        let signed_txn = Txn::new_signed(wallet_send, kp_recv.public_key(), 100, TxnType::Transfer);
        let msg = signed_txn.hash();

        // get signature
        let secp = Secp256k1::new();
        let signature: SecpEcdsaSignature =
            secp.sign_ecdsa(&Message::from_slice(&msg).unwrap(), &kp_send.secret_key());
        let answer = "3045022100fd15a048c36c5b3805da858e7a8a68d6c4bfd45450b0bf4bdfefcb7e92b553f30220445729607f6a2fb18c592922bd2bc44c0daf66fa43205820c9073c3b3568654f";

        assert!(signature.to_string() == answer.to_string(), "{signature:?}",);
    }
}
