pub mod getters;
pub mod setters;
pub mod utils;
// imports
use chrono::prelude::*;
use ed25519_dalek::Digest;
use serde::Serialize;
use serde_big_array::BigArray;
use std::fmt;
// local
use crate::{
    ledger::{
        general::{PbKey, Sha512},
        wallet::Wallet,
    },
    utils::signature::{TxnSignature, TXN_SIGNATURE_CONTEXT},
};
pub const TXN_MSG_CTX: &[u8; 6] = b"txn-v0";
pub const TXN_DIGEST_LEN: usize = 64;
pub type TxnDigest = [u8; 64];
pub type TxnCtxDigest = [u8; TXN_SIGNATURE_CONTEXT.len() + TXN_DIGEST_LEN];

// exported types
#[derive(Debug, Serialize, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct TxnId(#[serde(with = "BigArray")] pub TxnDigest);
impl From<Sha512> for TxnId {
    fn from(value: Sha512) -> Self {
        let val: TxnDigest = value.finalize().into();
        TxnId(val)
    }
}
impl From<TxnDigest> for TxnId {
    fn from(value: TxnDigest) -> Self {
        TxnId(value)
    }
}
impl From<TxnId> for TxnDigest {
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
    pub fn from_bytes(value: TxnDigest) -> Self {
        Self(value)
    }
    pub fn to_presigned_digest(&self) -> TxnCtxDigest {
        let mut digest_buffer: TxnCtxDigest = [0_u8; TXN_DIGEST_LEN + TXN_SIGNATURE_CONTEXT.len()];
        // add context
        digest_buffer[..TXN_SIGNATURE_CONTEXT.len()].copy_from_slice(TXN_SIGNATURE_CONTEXT);
        // add digest
        digest_buffer[TXN_SIGNATURE_CONTEXT.len()..self.0.len() + TXN_SIGNATURE_CONTEXT.len()]
            .copy_from_slice(&self.0);

        digest_buffer
    }
}
impl PartialEq<TxnDigest> for TxnId {
    #[inline]
    fn eq(&self, other: &TxnDigest) -> bool {
        constant_time_eq::constant_time_eq_64(&self.0, &other)
    }
}
impl PartialEq<TxnId> for TxnDigest {
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
    /// Creates a transaction `object`.
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

    /// ## Create and return a new signed transaction.
    /// Receives `Wallet` instance for signing.
    ///
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
}
