// external
use thiserror::Error;
// local
use super::p2p::P2PError;

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
