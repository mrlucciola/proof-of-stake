use anyhow::Result;
// imports
use chrono::prelude::*;
use rand::rngs;
use secp256k1::{
    ecdsa::Signature as EcdsaSignature,
    rand::SeedableRng,
    Message, // rand::{rngs, SeedableRng},
    PublicKey,
    // KeyPair, Secp256k1,
    Secp256k1,
};
// local
use super::wallet::Wallet;

#[derive(Debug)]
pub enum TxnType {
    Transfer,
}

#[derive(Debug)]
pub struct Txn {
    pub amt: u128,
    pub txn_type: TxnType,
    pub system_time: u64,
    pub recv_pubkey: PublicKey,
    pub sender_pubkey: PublicKey,
    /// Ecdsa
    pub signature: Option<EcdsaSignature>,
}

pub struct TxnHash();
pub struct TxnSignature();

impl Txn {
    /// Transaction constructor fxn
    ///
    /// creates a transaction `object`
    /// is public
    pub fn new(
        sender_pubkey: PublicKey,
        recv_pubkey: PublicKey,
        signature: Option<EcdsaSignature>,
        amt: u128,
        txn_type: TxnType,
    ) -> Self {
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();

        Self {
            sender_pubkey,
            recv_pubkey,
            amt,
            txn_type,
            system_time,
            signature,
        }
    }

    /// Analogous wrapper method for `get_hash`
    /// Creates a transaction hash from transaction body
    /// Must be a full and complete transaction body
    /// TODO: does not return anything
    pub fn hash(&self) -> TxnHash {
        Self::get_hash(self)
    }
    /// Creates a transaction hash from transaction body
    /// Must be a full and complete transaction body
    pub fn get_hash(txn_obj: &Txn) -> TxnHash {
        TxnHash()
    }

    /// 1) Sign the transaction
    /// 2) Add signature to transaction body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> Result<()> {
        let txn_signature = wallet.sign(self);

        // 2) add to txn body
        self.signature = Some(txn_signature);

        Ok(())
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
    let signature = None;
    Txn::new(sender_pubkey, recv_pubkey, signature, amt, txn_type)
}
pub fn create_sample_signed_txn() -> Txn {
    let secp = Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(10);
    let (_sender_privkey, sender_pubkey) = secp.generate_keypair(&mut rng);
    let (_recv_privkey, recv_pubkey) = secp.generate_keypair(&mut rng);

    let amt = 100;
    let txn_type = TxnType::Transfer;

    // get signature
    let secp = Secp256k1::new();
    let mut rng = rngs::StdRng::seed_from_u64(9);
    let (priv_key, _pub_key) = secp.generate_keypair(&mut rng);

    let msg_bytes = &[0xab; 32];
    let msg = Message::from_slice(msg_bytes).expect("trying to get txn message");
    let signature = secp.sign_ecdsa(&msg, &priv_key);
    Txn::new(sender_pubkey, recv_pubkey, Some(signature), amt, txn_type)
}
