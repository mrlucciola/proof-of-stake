// imports
use chrono::prelude::*;
use ed25519_dalek::{Digest, Sha512};
use serde::Serialize;
use std::fmt;
// local
use crate::{
    ledger::{general::PbKey, txn_pool::TxnMapKey, wallet::Wallet},
    utils::{hash::BlakeHash, signature::TxnSignature},
};
pub const TXN_MSG_CTX: &[u8; 6] = b"txn-v0";

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
    pub pbkey_send: [u8; 32],
    pub pbkey_recv: [u8; 32],
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
            pbkey_send: pbkey_send.to_bytes(),
            pbkey_recv: pbkey_recv.to_bytes(),
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
    #[deprecated = "Using byte arrays instead of strings."]
    pub fn id_str(&self) -> String {
        self.id().to_string()
    }
    /// Get `TxnMap` key type (derived from TxnId)
    /// @todo change to byte array
    pub fn id_key(&self) -> TxnMapKey {
        self.id().to_hex().to_string()
    }
    /// Getter for `Txn` `signature` property
    pub fn signature(&self) -> &TxnSignature {
        self.signature.as_ref().unwrap()
    }
    pub fn pbkey_send(&self) -> PbKey {
        PbKey::from_bytes(&self.pbkey_send).unwrap()
    }
    pub fn pbkey_recv(&self) -> PbKey {
        PbKey::from_bytes(&self.pbkey_recv).unwrap()
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
        let sig = wallet.sign_txn(self);
        self.set_signature(sig.clone());

        sig
    }

    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////

    /// Convert transaction struct to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing txn")
    }

    /// Compute the id (hash digest) of the transaction.
    ///
    /// Converts semantic data for the txn - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> TxnId {
        let mut hasher = blake3::Hasher::new();
        // add the txn version
        hasher.update(TXN_MSG_CTX);
        // add the txn bytes
        hasher.update(&self.to_bytes());
        // return the hash digest - the txn's id
        hasher.finalize().into()
    }
    pub fn calc_id_sha512(&self) -> ed25519_dalek::Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehashed: Sha512 = Sha512::new();

        // add the txn version
        prehashed.update(TXN_MSG_CTX);
        // add the txn bytes
        prehashed.update(self.to_bytes());

        prehashed
    }

    /// Create and return a message signature based on
    ///    the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> TxnSignature {
        wallet.sign_txn(self)
    }

    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}
