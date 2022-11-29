// imports
// use anyhow::Result as AResult;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};
// local
use crate::ledger::{general::PbKey, wallet::Wallet};
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
    pub pbkey_send: PbKey,
    pub pbkey_recv: PbKey,
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
    /// Creates a transaction `object`
    pub fn new(
        pbkey_send: PbKey,
        pbkey_recv: PbKey,
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
        wallet: &Wallet,
        pbkey_recv: PbKey,
        amt_to_send: u128,
        txn_type: TxnType,
    ) -> Txn {
        let pbkey_send = wallet.pbkey();
        let mut txn = Self::new(pbkey_send, pbkey_recv, amt_to_send, txn_type);

        // add signature to body
        txn.sign(&wallet);

        txn
    }
    /// Compute the hash digest of the transaction message - associated fxn
    pub fn get_hash(txn: &Txn) -> BlakeHash {
        // set blank vars
        let adj_txn_body = Txn {
            hash: [0u8; 32],
            signature: [0u8; 64],
            ..txn.clone()
        };

        // serialize to a byte vector
        let txn_msg_bytes: Vec<u8> = serde_json::to_vec(&adj_txn_body).unwrap();

        // get hash digest of txn
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"txn-v0");
        hasher.update(&txn_msg_bytes);
        let hash = hasher.finalize();

        hash
    }
    /// Method wrapper/analog for `get_hash()`
    pub fn hash(&self) -> TxnHash {
        Self::get_hash(&self).as_bytes().to_owned()
    }
    /// Get Txn map key (String) from byte array
    pub fn hash_str(&self) -> String {
        let hash = Self::get_hash(&self);
        hash.to_string()
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
        self.set_signature(sig);

        // 3) return signature
        sig
    }
}
