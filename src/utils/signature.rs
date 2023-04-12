#[derive(Debug, Clone, PartialEq)]
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
impl From<[u8; 64]> for TxnSignature {
    fn from(x: [u8; 64]) -> TxnSignature {
        TxnSignature(x.to_vec())
    }
}
impl From<TxnSignature> for [u8; 64] {
    fn from(x: TxnSignature) -> [u8; 64] {
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
#[derive(Debug, Clone)]
pub struct BlockSignature(pub Vec<u8>);
impl From<BlockSignature> for ed25519_dalek::Signature {
    fn from(x: BlockSignature) -> ed25519::Signature {
        ed25519::Signature::from_bytes(&x.0).unwrap()
    }
}
impl From<ed25519_dalek::Signature> for BlockSignature {
    fn from(x: ed25519_dalek::Signature) -> BlockSignature {
        BlockSignature(x.to_bytes().to_vec())
    }
}

pub type SignatureContextType = [u8; 32];

pub const BLOCK_SIGNATURE_CONTEXT: &SignatureContextType = b"ed25519BlkSignatureCtx3bx029zk3c";
