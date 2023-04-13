pub use ed25519_dalek::{Keypair as KP, PublicKey as PbKey, Sha512};
/// General reference regardless of which lib we use
pub use secp256k1::{Error as SecpError, SecretKey as PvKey};
use serde::Serialize;
pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, Serialize)]
pub struct PubKey([u8; 32]);

impl From<[u8; 32]> for PubKey {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}
impl From<PbKey> for PubKey {
    fn from(value: PbKey) -> Self {
        Self(value.to_bytes())
    }
}
