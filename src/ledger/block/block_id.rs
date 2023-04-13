// external
use {ed25519_dalek::Digest, serde::Serialize, serde_big_array::BigArray};
// local
use super::types::BlockDigest;
use crate::ledger::general::Sha512;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(#[serde(with = "BigArray")] pub BlockDigest);
impl From<Sha512> for BlockId {
    fn from(value: Sha512) -> Self {
        let val: BlockDigest = value.finalize().into();
        BlockId(val)
    }
}
impl From<BlockDigest> for BlockId {
    fn from(value: BlockDigest) -> Self {
        BlockId(value)
    }
}
impl From<BlockId> for BlockDigest {
    fn from(value: BlockId) -> Self {
        value.0
    }
}
impl BlockId {
    pub fn from_bytes(value: BlockDigest) -> Self {
        Self(value)
    }
}
impl PartialEq<BlockDigest> for BlockId {
    #[inline]
    fn eq(&self, other: &BlockDigest) -> bool {
        constant_time_eq::constant_time_eq_64(&self.0, other)
    }
}
