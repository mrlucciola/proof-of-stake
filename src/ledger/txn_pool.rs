use crate::ledger::{general::Result, txn::Txn};
use {
    serde::{Deserialize, Serialize},
    std::collections::BTreeMap,
};

// export types
// @todo move to a more appropriate location
pub type TxnMapKey = String;
pub type TxnMap = BTreeMap<TxnMapKey, Txn>;

/// ## Data structure which holds all pending transactions
#[derive(Debug, Serialize, Deserialize)]
pub struct TxnPool {
    /// Ordered lookup of transactions
    values: TxnMap,
}
impl TxnPool {
    /// ### Initialize new transaction pool.
    pub fn new() -> Self {
        let txns = TxnMap::new();

        Self { values: txns }
    }
    /// ### Check if a transaction exists in the txn pool (#7).
    ///
    /// Use `txn id` to query the pool, return true if it exists.
    pub fn does_txn_exist(&self, txn: &Txn) -> bool {
        match self.values.get(&txn.id_key()) {
            Some(_) => true,
            None => false,
        }
    }
    /// ### Append a transaction to the pool.
    ///
    /// Check for duplicate and handle properly.
    /// - @todo verify the requesting node is authorized
    /// - @todo validate signature
    pub fn add_txn(&mut self, txn: Txn) -> Result<()> {
        // if value (txn from pool) is returned from the `.insert()` call, then this txn already exists in the pool - throw error
        if let Some(_txn) = self.values.insert(txn.id_key().to_owned(), txn) {
            return Err(TxnPoolError::DuplicateTxn.into());
        }

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////
    /// ### Get property `TxnPool.txns`.
    pub fn txns(&self) -> &TxnMap {
        &self.values
    }
    /// ### Get the number of transactions in the pool.
    /// Length of the collection of transactions.
    pub fn txn_ct(&self) -> usize {
        self.values.len()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////
    /// ### Remove and return a transaction from pool.
    ///
    /// Use the Txn Map Key to look up the transaction in the transaction pool.\
    /// Calls `BTreeMap.remove()`.
    /// - @todo (unsure about this) verify the requesting node is authorized
    pub fn remove_txn(&mut self, txn: &Txn) -> Result<Txn> {
        match self.values.remove(&txn.id_key()) {
            Some(txn) => Ok(txn),
            None => Err(TxnPoolError::TxnDoesNotExist.into()),
        }
    }
    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////
}

// @todo move to a more appropriate location
#[derive(Debug, thiserror::Error)]
pub enum TxnPoolError {
    #[error("Attempting to add duplicate txn.")]
    DuplicateTxn,
    #[error("Transaction does not exist in pool.")]
    TxnDoesNotExist,
}
