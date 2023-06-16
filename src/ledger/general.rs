use serde::{Deserialize, Serialize};
// aliased types
/// General reference regardless of which lib we use
pub type HashAlgo = ed25519_dalek::Sha512;
pub type KP = ed25519_dalek::Keypair;
pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PbKey(pub [u8; 32]);
impl From<PbKey> for [u8; 32] {
    fn from(value: PbKey) -> Self {
        value.0
    }
}
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

impl From<PbKey> for libp2p::identity::PublicKey {
    fn from(value: PbKey) -> Self {
        let pk = libp2p::identity::ed25519::PublicKey::decode(&value.0).unwrap();
        let pk: libp2p::identity::PublicKey = libp2p::identity::PublicKey::Ed25519(pk);

        pk
    }
}

impl From<libp2p::identity::PublicKey> for PbKey {
    #[allow(unreachable_patterns)]
    fn from(value: libp2p::identity::PublicKey) -> Self {
        let pk = match value {
            libp2p::identity::PublicKey::Ed25519(pk) => pk,
            _ => panic!("Unsupported public key type"),
        };
        Self(pk.encode())
    }
}
