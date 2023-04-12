// imports
use chrono::prelude::*;
use ed25519_dalek::{Digest, Sha512};
use serde::Serialize;
use serde_big_array::BigArray;
use std::fmt;
// local
use crate::{
    ledger::{general::PbKey, txn_pool::TxnMapKey, wallet::Wallet},
    utils::signature::TxnSignature,
};
pub const TXN_MSG_CTX: &[u8; 6] = b"txn-v0";

// exported types
#[derive(Debug, Serialize, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct TxnId(#[serde(with = "BigArray")] pub [u8; 64]);
impl From<Sha512> for TxnId {
    fn from(value: Sha512) -> Self {
        let val: [u8; 64] = value.finalize().into();
        TxnId(val)
    }
}
impl From<[u8; 64]> for TxnId {
    fn from(value: [u8; 64]) -> Self {
        TxnId(value)
    }
}
impl From<TxnId> for [u8; 64] {
    fn from(value: TxnId) -> Self {
        value.0
    }
}
impl From<TxnId> for String {
    fn from(value: TxnId) -> Self {
        hex::encode(value.0.as_ref())
    }
}
impl TxnId {
    pub fn from_bytes(value: [u8; 64]) -> Self {
        Self(value)
    }
}
impl PartialEq<[u8; 64]> for TxnId {
    #[inline]
    fn eq(&self, other: &[u8; 64]) -> bool {
        constant_time_eq::constant_time_eq_64(&self.0, &other)
    }
}
impl PartialEq<TxnId> for [u8; 64] {
    #[inline]
    fn eq(&self, other: &TxnId) -> bool {
        constant_time_eq::constant_time_eq_64(&self, &other.0)
    }
}
impl PartialEq for TxnId {
    #[inline]
    fn eq(&self, other: &TxnId) -> bool {
        constant_time_eq::constant_time_eq_64(&self.0, &other.0)
    }
}

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
    /// ## Transaction constructor fxn
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
            id: None,        //[0u8; 64],
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

    /// ## Get `Txn.id` property.
    /// Panic when accessing unset value.
    pub fn id(&self) -> TxnId {
        self.id.unwrap()
    }
    /// ### Get `TxnMap` key type (derived from TxnId).
    /// @todo change to byte array
    pub fn id_key(&self) -> TxnMapKey {
        self.id().into()
    }
    /// ### Getter for `Txn` `signature` property
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

    /// ## Get identifier (hash) for txn and set on txn object and store the output on the Txn object
    ///
    /// Returns id
    pub fn set_id(&mut self) -> TxnId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }
    /// ## Set the signature for the transaction.
    pub fn set_signature(&mut self, signature: TxnSignature) {
        self.signature = Some(signature);
    }

    /// ## Add the signature to the transaction body in place.
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
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing txn")
    }

    /// ## Compute the id (hash digest) of the transaction.
    /// Converts semantic data for the txn - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> TxnId {
        let prehash = self.calc_id_sha512_prehash();

        // return the hash digest - the block's id
        let digest: [u8; 64] = prehash.finalize().into();
        TxnId(digest)
    }
    /// ## Calculate the pre-hash struct for the id
    pub fn calc_id_sha512_prehash(&self) -> ed25519_dalek::Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehash: Sha512 = Sha512::new();
        // add the txn version
        prehash.update(TXN_MSG_CTX);
        // add the txn bytes
        prehash.update(self.to_bytes());
        // return the hasher/prehash struct
        prehash
    }

    /// ## Create and return a message signature based on the contents of the transaction
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
