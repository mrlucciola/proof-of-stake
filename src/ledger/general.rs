/// General reference regardless of which lib we use
pub use secp256k1::{Error as SecpError, KeyPair as KP, PublicKey as PbKey, SecretKey as PvKey};
pub type Result<T> = std::result::Result<T, anyhow::Error>;
