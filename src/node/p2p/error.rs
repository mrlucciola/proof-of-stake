#[derive(Debug, thiserror::Error)]
pub enum P2PError {
    #[error("Misc P2P error.")]
    P2P,
    #[error("P2P module not initialized.")]
    InitP2P,
    #[error("IoError.")]
    IoError(#[from] std::io::Error),
}
