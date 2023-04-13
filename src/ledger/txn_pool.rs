// import
use serde::Serialize;
use std::collections::BTreeMap;
// local
use crate::ledger::{general::Result, txn::Txn};

// export types
pub type TxnMapKey = String; // TODO?: change to hex
pub type TxnMap = BTreeMap<TxnMapKey, Txn>;

/// Data structure which holds all pending transactions
#[derive(Debug, Serialize)]
pub struct TxnPool {
    // Array of transactions
    values: TxnMap,
}
impl TxnPool {
    /// Initializer for Transaction Pool
    ///
    /// Create a data structure which
    pub fn new() -> Self {
        let txns = TxnMap::new();

        Self { values: txns }
    }
    /// Check if a transaction exists in the txn pool (#7)
    ///
    /// Use txn id to query the pool, return true if it exists
    pub fn does_txn_exist(&self, txn: &Txn) -> bool {
        match self.values.get(&txn.id_key()) {
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
        // @todo verify the requesting node is authorized
        // @todo validate signature

        // add txn to pool
        if let Some(_txn) = self.values.insert(txn.id_key().to_owned(), txn) {
            return Err(TxnPoolError::DuplicateTxn.into());
        }

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// Getter
    pub fn txns(&self) -> &TxnMap {
        &self.values
    }
    /// ## Get the number of transactions in the pool.
    pub fn txn_ct(&self) -> usize {
        self.values.len()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// ## Remove and return a transaction from pool.
    ///
    /// Use the Txn Map Key to look up the transaction in the transaction pool.
    ///
    /// Calls `BTreeMap.remove()`
    pub fn remove_txn(&mut self, txn: &Txn) -> Result<Txn> {
        // TODO: verify the requesting node is authorized
        match self.values.remove(&txn.id_key()) {
            Some(txn) => Ok(txn),
            // @todo handle error (in comments below)
            None => Err(anyhow::format_err!("NoTxn")), // TODO: create proper txn error
        }
    }

    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////

    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}

#[derive(thiserror::Error, Debug)]
pub enum TxnPoolError {
    #[error("Attempting to add duplicate txn.")]
    DuplicateTxn,
}
