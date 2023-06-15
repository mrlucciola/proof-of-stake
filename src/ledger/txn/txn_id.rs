// external
use {
    ed25519_dalek::Digest,
    serde::{Deserialize, Serialize},
    serde_big_array::BigArray,
};

// local
use super::{TxnCtxDigest, TxnDigest, TXN_DIGEST_LEN, TXN_SIGNATURE_CTX};
use crate::ledger::general::Sha512;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
impl From<&TxnId> for String {
    fn from(value: &TxnId) -> Self {
        hex::encode(value.0)
    }
}
impl TxnId {
    pub fn from_bytes(value: TxnDigest) -> Self {
        Self(value)
    }
    pub fn to_presigned_digest(&self) -> TxnCtxDigest {
        let mut digest_buffer: TxnCtxDigest = [0_u8; TXN_DIGEST_LEN + TXN_SIGNATURE_CTX.len()];
        // add context
        digest_buffer[..TXN_SIGNATURE_CTX.len()].copy_from_slice(TXN_SIGNATURE_CTX);
        // add digest
        digest_buffer[TXN_SIGNATURE_CTX.len()..self.0.len() + TXN_SIGNATURE_CTX.len()]
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
