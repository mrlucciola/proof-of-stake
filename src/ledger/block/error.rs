// external
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockError {
    #[error("BlockError::EmptySignature- Invalid block: No signature")]
    EmptySignature,
    #[error("BlockError::EmptyId- Invalid block: No ID")]
    EmptyId,
    #[error("BlockError::IncorrectId- Incorrect ID")]
    IncorrectId,
    #[error("BlockError::InvalidSignature- {0}. Testing signature:\n{1}")]
    InvalidSignature(ed25519_dalek::SignatureError, ed25519::Signature),
}
