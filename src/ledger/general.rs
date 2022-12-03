#[deprecated(note="use the one in signatures")]
pub type SecpEcdsaSignature = secp256k1::ecdsa::Signature;

/// General reference regardless of which lib we use
pub use secp256k1::{KeyPair as KP, PublicKey as PbKey, SecretKey as PvKey};
pub type Result<T> = std::result::Result<T, anyhow::Error>;
