use crate::ledger::{block::types::BlockDigest, general::HashAlgo};
use {
    ed25519_dalek::Digest,
    serde::{Deserialize, Serialize},
    serde_big_array::BigArray,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockId(#[serde(with = "BigArray")] pub BlockDigest);

impl BlockId {
    pub fn from_bytes(value: BlockDigest) -> Self {
        Self(value)
    }
    pub fn to_str(&self) -> String {
        hex::encode(self.0)
    }
}

impl From<HashAlgo> for BlockId {
    fn from(value: HashAlgo) -> Self {
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
impl PartialEq<BlockDigest> for BlockId {
    #[inline]
    fn eq(&self, other: &BlockDigest) -> bool {
        constant_time_eq::constant_time_eq_64(&self.0, other)
    }
}
