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
    txns: TxnMap,
}
impl TxnPool {
    /// Initializer for Transaction Pool
    ///
    /// Create a data structure which
    pub fn new() -> Self {
        let txns = TxnMap::new();

        Self { txns }
    }
    /// Check if a transaction exists in the txn pool (#7)
    ///
    /// Use txn id to query the pool, return true if it exists
    pub fn does_txn_exist(&self, txn: &Txn) -> bool {
        match self.txns.get(&txn.id_key()) {
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
        self.txns.entry(txn.id_key()).or_insert(txn);

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// Getter
    pub fn txns(&self) -> &TxnMap {
        &self.txns
    }
    /// Convenience function
    pub fn txn_ct(&self) -> usize {
        self.txns.len()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// Remove a transaction from the pool by its id (hash)
    ///
    /// Calls remove_txn
    pub fn remove_txn(&mut self, txn: &Txn) -> Result<Txn> {
        // TODO: verify the requesting node is authorized
        match self.txns.remove(&txn.id_key()) {
            Some(txn) => Ok(txn),
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
