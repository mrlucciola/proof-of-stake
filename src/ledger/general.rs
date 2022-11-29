pub use secp256k1::ecdsa::Signature as SecpEcdsaSignature;

/// General reference regardless of which lib we use
pub use secp256k1::{KeyPair as KP, PublicKey as PbKey, SecretKey as PvKey};
