// imports
use chrono::prelude::*;

use super::wallet::Wallet;

#[derive(Debug)]
pub enum TxnType {
    Transfer,
}

#[derive(Debug)]
pub struct Txn {
    amt: u128,
    txn_type: TxnType,
    system_time: u64,
    recv_pubkey: String,
    sender_pubkey: String,
    signature: String,
}

pub struct TxnHash();
pub struct TxnSignature();

impl Txn {
    /// Transaction constructor fxn
    ///
    /// creates a transaction `object`
    /// is public
    pub fn new(
        sender_pubkey: String,
        recv_pubkey: String,
        signature: String,
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

    ///
    pub fn hash(_data: &Txn) -> TxnHash {
        TxnHash()
    }

    ///
    pub fn signature(_txn_hash: TxnHash) -> TxnSignature {
        TxnSignature()
    }

    /// Takes the txn data object and returns a signature
    pub fn sign(&self, _wallet: Wallet) -> TxnSignature {
        let txn_hash = Self::hash(self);

        Self::signature(txn_hash)
    }
}

pub fn create_sample_txn() -> Txn {
    let sender_pubkey = String::from("1");
    let recv_pubkey = String::from("1");
    let amt = 100;
    let txn_type = TxnType::Transfer;
    let signature = String::from("");

    Txn::new(sender_pubkey, recv_pubkey, signature, amt, txn_type)
}
