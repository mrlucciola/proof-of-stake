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
