use serde::{Deserialize, Serialize};

use super::TxnDigest;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TxnSignature(pub Vec<u8>);
impl From<TxnSignature> for ed25519_dalek::Signature {
    fn from(x: TxnSignature) -> ed25519::Signature {
        ed25519::Signature::from_bytes(&x.0).unwrap()
    }
}
impl From<ed25519_dalek::Signature> for TxnSignature {
    fn from(x: ed25519_dalek::Signature) -> TxnSignature {
        TxnSignature(x.to_bytes().to_vec())
    }
}
impl From<TxnDigest> for TxnSignature {
    fn from(x: TxnDigest) -> TxnSignature {
        TxnSignature(x.to_vec())
    }
}
impl From<TxnSignature> for TxnDigest {
    fn from(x: TxnSignature) -> TxnDigest {
        x.0.try_into().unwrap()
    }
}
impl From<TxnSignature> for String {
    fn from(x: TxnSignature) -> Self {
        hex::encode(x.0)
    }
}
impl TxnSignature {
    pub fn to_str(&self) -> String {
        hex::encode(&self.0)
    }
}
