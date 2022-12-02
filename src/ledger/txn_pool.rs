// import
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// local
use crate::ledger::{
    general::Result,
    txn::{Txn, TxnId},
};

// export types
pub type PoolTxnMap = HashMap<TxnId, Txn>;

/// Data structure which holds all pending transactions
#[derive(Debug, Serialize, Deserialize)]
pub struct TxnPool {
    // Array of transactions
    txns: PoolTxnMap,
}
impl TxnPool {
    /// Initializer for Transaction Pool
    ///
    /// Create a data structure which
    pub fn new() -> Self {
        let txns = PoolTxnMap::new();

        Self { txns }
    }
    /// Check if a transaction exists in the txn pool (#7)
    ///
    /// Use txn id to query the pool, return true if it exists
    pub fn does_txn_exist(&self, &txn_id: &TxnId) -> bool {
        match self.txns.get(&txn_id) {
            Some(_) => true,
            None => false,
        }
    }
    /// Add a transaction to the pool.
    ///
    /// Check for duplicate and handle properly.
    ///
    /// This should be completed by any RPC/relayer
    pub fn add_txn(&mut self, txn: Txn) -> Result<()> {
        // TODO: verify the requesting node is authorized

        // add txn to pool
        self.txns.entry(txn.id).or_insert(txn);

        Ok(())
    }

    /// Remove a transaction from the pool by its id (hash)
    ///
    /// Calls remove_txn
    pub fn remove_txn(&mut self, txn_id: &TxnId) -> Result<Txn> {
        // TODO: verify the requesting node is authorized
        match self.txns.remove(txn_id) {
            Some(txn) => Ok(txn),
            None => Err(anyhow::format_err!("NoTxn")), // TODO: create proper txn error
        }
    }
    pub fn txns(&self) -> &PoolTxnMap {
        &self.txns
    }
    pub fn txn_ct(&self) -> usize {
        self.txns.len()
    }
}
