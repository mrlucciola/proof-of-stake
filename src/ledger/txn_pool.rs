use secp256k1::Message;

use super::txn::Txn;
pub type Result<T> = std::result::Result<T, failure::Error>;

/// Data structure which holds all pending transactions
#[derive(Debug)]
struct TxnPool {
    /// Array of transactions
    txns: Vec<Txn>,
}
/// TODO: create `add_txn` method
/// TODO: create `remove_txn` method
impl TxnPool {
    /// Initializer for Transaction Pool
    ///
    /// Create a data structure which
    pub fn new() -> Self {
        TxnPool { txns: vec![] }
    }
    /// Add a transaction to the pool
    ///
    /// This should be completed by any RPC/relayer
    pub fn add_txn(&mut self, txn: Txn) -> Result<()> {
        // TODO: verify the requesting node is authorized

        // add txn to pool
        self.txns.push(txn);

        Ok(())
    }

    /// Remove a transaction from the pool by its hash
    ///
    /// Calls remove_txn
    pub fn remove_txn(&mut self, txn_hash: &Message) -> Result<()> {
        // TODO: verify the requesting node is authorized

        let txn_iter: Vec<&Txn> = self
            .txns
            .iter()
            .filter(|&v| &v.hash.unwrap() == txn_hash)
            .collect();

        // verify no dups
        match txn_iter.len() {
            // fine
            1 => {}
            0 => return Err(failure::err_msg("NotFound")),
            x if x > 1 => return Err(failure::err_msg("DuplicateTxn")),
            _ => return Err(failure::err_msg("UnknownError")),
        }
        self.txns.retain(|txn| &txn.hash.unwrap() != txn_hash);

        Ok(())
    }
}

mod tests {
    use super::*;
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
        let hash = txn_1.get_txn_msg();

        // remove from pool
        txn_pool.remove_txn(&hash)?;
        assert!(txn_pool.txns.len() == 0);

        Ok(())
    }
}
