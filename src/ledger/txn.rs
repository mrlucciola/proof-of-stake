use std::fmt;

// imports
use chrono::prelude::*;
use serde::Serialize;
// local
use crate::{
    ledger::{general::PbKey, txn_pool::TxnMapKey, wallet::Wallet},
    utils::{
        hash::{BlakeHash, BlakeHex},
        signature::TxnSignature,
    },
};

// exported types
pub type TxnId = BlakeHash;

#[derive(Serialize, Debug, Clone, Copy, PartialEq)]
pub enum TxnType {
    Transfer = 1,
}
impl fmt::Display for TxnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// ## Transfer transaction.
///
/// Serializable body of the transaction.
///
/// @todo generalize this and abstract all separate types.\
/// @todo make fields private and add getters.
#[derive(Serialize, Debug, Clone)]
pub struct Txn {
    pub amt: u128,
    pub pbkey_send: PbKey,
    pub pbkey_recv: PbKey,
    // The time the txn was created
    pub system_time: u64,
    /// Type of transaction - as int
    pub txn_type: TxnType,
    /// Transaction identifier: Blake3 hash (currently as byte array)
    #[serde(skip_serializing)]
    id: Option<TxnId>,
    /// Ecdsa signature as byte array
    #[serde(skip_serializing)]
    signature: Option<TxnSignature>,
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
            id: None,        //[0u8; 32],
            signature: None, //[0u8; 64],
        };

        // set the id with the body
        txn.set_id();
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

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// Getter
    pub fn id(&self) -> TxnId {
        self.id.unwrap()
    }
    /// Get Txn Id in `String` form.
    pub fn id_str(&self) -> String {
        self.id().to_string()
    }
    /// Get Txn Id in `hex` form.
    pub fn id_hex(&self) -> BlakeHex {
        self.id().to_hex()
    }
    /// Get Txn Id Hex in `String` form.
    pub fn id_hex_string(&self) -> String {
        self.id().to_hex().to_string()
    }
    /// Get `TxnMap` key type (derived from TxnId)
    pub fn id_key(&self) -> TxnMapKey {
        self.id_hex_string()
    }
    /// Getter for `Txn` `signature` property
    pub fn signature(&self) -> &TxnSignature {
        self.signature.as_ref().unwrap()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// Get identifier (hash) for txn and set on txn object and store the output on the Txn object
    ///
    /// Returns id
    pub fn set_id(&mut self) -> TxnId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }

    pub fn set_signature(&mut self, signature: TxnSignature) {
        self.signature = Some(signature);
    }

    /// Add the signature to the transaction body in place.
    ///
    /// 1) Sign the transaction
    /// 2) Add signature to transaction body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> TxnSignature {
        let sig = self.calc_signature(wallet);
        self.set_signature(sig.clone());

        sig
    }

    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////

    /// Convert transaction struct to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate
    pub fn as_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing txn")
    }

    /// Create and return a message signature based on
    ///    the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> TxnSignature {
        let msg: TxnId = self.id();

        wallet.sign_txn(&msg)
    }

    /// Compute the id (hash digest) of the transaction.
    ///
    /// Converts semantic data for the block - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> TxnId {
        let mut hasher = blake3::Hasher::new();
        // add the block version
        hasher.update(b"txn-v0");
        // add the block bytes
        hasher.update(&self.as_bytes());
        // return the hash digest - the block's id
        hasher.finalize().into()
    }

    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}
