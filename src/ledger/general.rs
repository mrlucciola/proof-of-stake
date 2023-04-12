/// General reference regardless of which lib we use
pub use secp256k1::{Error as SecpError, KeyPair as KP, SecretKey as PvKey};
pub use ed25519_dalek::{PublicKey as PbKey, Sha512};
pub type Result<T> = std::result::Result<T, anyhow::Error>;
