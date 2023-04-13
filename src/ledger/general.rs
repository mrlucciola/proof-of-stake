/// General reference regardless of which lib we use
pub use ed25519_dalek::{Keypair as KP, Sha512};
pub use secp256k1::{Error as SecpError, SecretKey as PvKey};
use serde::Serialize;
pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct PbKey(pub [u8; 32]);
impl From<PbKey> for [u8; 32] {
    fn from(value: PbKey) -> Self {
        value.0
    }
}
// impl From<PbKey> for &'static [u8; 32] {
//     fn from(value: PbKey) -> &'static [u8; 32] {
//         value.0.as_ref()
//     }
// }
impl From<&PbKey> for [u8; 32] {
    fn from(value: &PbKey) -> Self {
        value.0.to_owned()
    }
}
impl From<[u8; 32]> for PbKey {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}
impl From<ed25519_dalek::PublicKey> for PbKey {
    fn from(value: ed25519_dalek::PublicKey) -> Self {
        Self(value.to_bytes())
    }
}
impl From<PbKey> for ed25519_dalek::PublicKey {
    fn from(value: PbKey) -> Self {
        ed25519_dalek::PublicKey::from_bytes(&value.0).unwrap()
    }
}
impl From<&PbKey> for ed25519_dalek::PublicKey {
    fn from(value: &PbKey) -> Self {
        ed25519_dalek::PublicKey::from_bytes(&value.0).unwrap()
    }
}
