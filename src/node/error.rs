use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Blockchain not initialized.")]
    InitBlockchain,
    #[error("Wallet not initialized.")]
    InitWallet,
    #[error("Transaction pool not initialized.")]
    InitTxnPool,
}

// #[deprecated(note = "Replace with Txn Map")]
