// imports
use anyhow::Result as AResult;
use chrono::prelude::*;
use secp256k1::{
    ecdsa::Signature as TxnSignature,
    rand::{rngs, SeedableRng},
    Message, PublicKey, Secp256k1,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// local
use super::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug)]
pub enum TxnType {
    Transfer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxnBody {
    pub amt: u128,
    pub txn_type: TxnType,
    /// timestamp
    pub system_time: u64,
    pub recv_pubkey: PublicKey,
    pub sender_pubkey: PublicKey,
    pub id: String,
}

// TODO: turn this into a full-fledged wrapper around txn data
#[derive(Serialize, Deserialize, Debug)]
pub struct Txn {
    pub body: TxnBody,
    /// Ecdsa
    pub signature: Option<TxnSignature>,
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
            txn_type,
            system_time, // timestamp
            id,
        };
        Self {
            body,
            signature: Option::None,
        }
    }

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

pub fn create_sample_unsigned_txn() -> Txn {
    let secp = Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(10);
    let (_sender_privkey, sender_pubkey) = secp.generate_keypair(&mut rng);
    let (_recv_privkey, recv_pubkey) = secp.generate_keypair(&mut rng);

    let amt = 100;
    let txn_type = TxnType::Transfer;

    // get signature
    Txn::new(sender_pubkey, recv_pubkey, amt, txn_type)
}
pub fn create_sample_signed_txn() -> Txn {
    let secp = Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(10);
    let (_sender_privkey, sender_pubkey) = secp.generate_keypair(&mut rng);
    let (_recv_privkey, recv_pubkey) = secp.generate_keypair(&mut rng);

    let amt = 100;
    let txn_type = TxnType::Transfer;

    // turn the raw txn into message
    let mut txn = Txn::new(sender_pubkey, recv_pubkey, amt, txn_type);
    let msg = txn.get_txn_msg();

    // get signature
    let secp = Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(9);
    let (priv_key, _pub_key) = secp.generate_keypair(&mut rng);

    let signature = secp.sign_ecdsa(&msg, &priv_key);
    txn.signature = Some(signature);

    txn
}
