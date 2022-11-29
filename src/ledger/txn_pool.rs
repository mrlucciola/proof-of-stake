// import
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// local
use crate::ledger::txn::{Txn, TxnHash};
// export types
pub type Result<T> = std::result::Result<T, failure::Error>;
pub type PoolTxnMap = HashMap<TxnHash, Txn>;

/// Data structure which holds all pending transactions
#[derive(Debug, Serialize, Deserialize)]
struct TxnPool {
    // Array of transactions
    pub txns: PoolTxnMap,
}
impl TxnPool {
    /// Initializer for Transaction Pool
    ///
    /// Create a data structure which
    pub fn new() -> Self {
        let new_thing = PoolTxnMap::new();

        TxnPool {
            txns: PoolTxnMap::new(),
        }
    }
    /// Check if a transaction exists in the txn pool (#7)
    ///
    /// Use txn hash to query the pool, return true if it exists
    pub fn does_txn_exist(&self, &txn_hash: &TxnHash) -> bool {
        match self.txns.get(&txn_hash) {
            Some(x) => true,
            None => false,
        }
    }
    /// Add a transaction to the pool
    ///
    /// This should be completed by any RPC/relayer
    pub fn add_txn(&mut self, txn: Txn) -> Result<()> {
        // TODO: verify the requesting node is authorized

        // add txn to pool
        self.txns.entry(txn.hash).or_insert(txn);

        Ok(())
    }

    /// Remove a transaction from the pool by its hash
    ///
    /// Calls remove_txn
    pub fn remove_txn(&mut self, txn_hash: &TxnHash) -> Result<Txn> {
        // TODO: verify the requesting node is authorized

        match self.txns.remove(txn_hash) {
            Some(txn) => Ok(txn),
            None => Err(failure::err_msg("NoTxn")), // TODO: create proper txn error
        }

        // Ok(())
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::ledger::{txn::TxnType, wallet::Wallet};

    #[test]
    fn create_txn_pool() {
        let txn_pool = TxnPool::new();
        assert!(txn_pool.txns.len() == 0);
    }
    #[test]
    fn add_txn_PASS() -> Result<()> {
        // init txn pool
        let mut txn_pool = TxnPool::new();
        let pbkey1 = Wallet::new_from_file(&"./test_key.json".to_string()).get_pbkey();
        let pbkey2 = Wallet::new_from_file(&"./test_key_recv.json".to_string()).get_pbkey();

        // create txn
        let txn_1 = Txn::new(pbkey1, pbkey2, 100, TxnType::Transfer);
        // add to pool
        assert!(txn_pool.txns.len() == 0);
        txn_pool.add_txn(txn_1)?;

        assert!(txn_pool.txns.len() == 1);

        Ok(())
    }
    #[test]
    fn add_txn_FAIL_dup() -> Result<()> {
        // init txn pool
        let mut txn_pool = TxnPool::new();
        let pbkey1 = Wallet::new_from_file(&"./test_key.json".to_string()).get_pbkey();
        let pbkey2 = Wallet::new_from_file(&"./test_key_recv.json".to_string()).get_pbkey();

        // create txn
        let txn_1 = Txn::new(pbkey1, pbkey2, 100, TxnType::Transfer);
        // add to pool
        txn_pool.add_txn(txn_1)?;
        // should fail
        txn_pool.add_txn(txn_1)?;
        assert!(txn_pool.txns.len() == 1);

        Ok(())
    }
    #[test]
    fn remove_txn_PASS() -> Result<()> {
        // init txn pool
        let mut txn_pool = TxnPool::new();
        let pbkey1 = Wallet::new_from_file(&"./test_key.json".to_string()).get_pbkey();
        let pbkey2 = Wallet::new_from_file(&"./test_key_recv.json".to_string()).get_pbkey();
        // create txn
        let txn_1 = Txn::new(pbkey1, pbkey2, 100, TxnType::Transfer);

        // add to pool
        txn_pool.add_txn(txn_1.clone())?;
        assert!(txn_pool.txns.len() == 1);
        let hash = txn_1.hash();

        // remove from pool
        txn_pool.remove_txn(&hash)?;
        assert!(txn_pool.txns.len() == 0);

        Ok(())
    }
    #[test]
    fn does_txn_exist_PASS() -> Result<()> {
        // init txn pool
        let mut txn_pool = TxnPool::new();
        let pbkey1 = Wallet::new_from_file(&"./test_key.json".to_string()).get_pbkey();
        let pbkey2 = Wallet::new_from_file(&"./test_key_recv.json".to_string()).get_pbkey();
        // create txn
        let txn_1 = Txn::new(pbkey1, pbkey2, 100, TxnType::Transfer);

        // add to pool
        txn_pool.add_txn(txn_1.clone())?;
        assert!(txn_pool.txns.len() == 1);
        let hash = txn_1.hash;

        assert!(txn_pool.does_txn_exist(&hash));

        Ok(())
    }
}
