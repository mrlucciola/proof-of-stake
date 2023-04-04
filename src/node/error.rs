use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Blockchain not initialized.")]
    InitBlockchain,
    #[error("Wallet not initialized.")]
    InitWallet,
    #[error("Transaction pool not initialized.")]
    InitTxnPool,
    #[error("P2P")]
    P2PError(#[from] P2PError),
}

#[derive(Error, Debug)]
pub enum P2PError {
    #[error("Misc P2P error.")]
    P2P,
    #[error("P2P module not initialized.")]
    InitP2P,
    #[error("IoError")]
    IoError(#[from] std::io::Error),
}
