// external
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockchainError {
    /// @todo move to `Txn`
    #[allow(dead_code)]
    #[error("Total balance before and after transaction do not match.")]
    TransactionBalanceMismatch,
    /// @todo move to `Txn`
    #[allow(dead_code)]
    #[error("Account balance does not change by amount determined by txn.")]
    AccountBalanceChangeMismatch,
}
