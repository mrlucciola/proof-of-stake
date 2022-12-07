// imports
use std::fmt::Debug;
// local

/// We may switch the library used for signatures, using this abstraction preemptively to semantically separate `signature` types.
#[derive(Debug, Clone)]
pub struct SecpEcdsaSignature(pub(crate) secp256k1::ecdsa::Signature);
impl From<secp256k1::ecdsa::Signature> for SecpEcdsaSignature {
    fn from(t: secp256k1::ecdsa::Signature) -> SecpEcdsaSignature {
        SecpEcdsaSignature(t)
    }
}

impl SecpEcdsaSignature {
    // Serializes the signature in compact format
    pub fn serialize_compact(&self) -> [u8; 64] {
        secp256k1::ecdsa::Signature::serialize_compact(&self.0)
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl From<SecpEcdsaSignature> for secp256k1::ecdsa::Signature {
    fn from(x: SecpEcdsaSignature) -> secp256k1::ecdsa::Signature {
        x.0
    }
}

pub type TxnSignature = secp256k1::ecdsa::Signature;
pub type BlockSignature = secp256k1::ecdsa::Signature;
