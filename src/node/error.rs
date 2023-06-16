use crate::node::p2p::error::P2PError;

#[derive(Debug, thiserror::Error)]
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
