// external
use {ed25519_dalek::Digest, serde::Serialize, serde_big_array::BigArray};
// local
use crate::ledger::general::Sha512;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(#[serde(with = "BigArray")] pub [u8; 64]);
impl From<Sha512> for BlockId {
    fn from(value: Sha512) -> Self {
        let val: [u8; 64] = value.finalize().into();
        BlockId(val)
    }
}
impl From<[u8; 64]> for BlockId {
    fn from(value: [u8; 64]) -> Self {
        BlockId(value)
    }
}
impl From<BlockId> for [u8; 64] {
    fn from(value: BlockId) -> Self {
        value.0
    }
}
impl BlockId {
    pub fn from_bytes(value: [u8; 64]) -> Self {
        Self(value)
    }
}
impl PartialEq<[u8; 64]> for BlockId {
    #[inline]
    fn eq(&self, other: &[u8; 64]) -> bool {
        constant_time_eq::constant_time_eq_64(&self.0, other)
    }
}
