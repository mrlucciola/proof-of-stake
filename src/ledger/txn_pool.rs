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
}

mod tests {
    use super::*;
    use crate::ledger::{txn::TxnType, wallet::Wallet};

    #[test]
    fn create_txn_pool() {
        use super::*;

        let mut txn_pool = TxnPool::new();
        let pbkey1 = Wallet::new_from_file(&"./test_key.json".to_string()).get_pbkey();
        let pbkey2 = Wallet::new_from_file(&"./test_key_recv.json".to_string()).get_pbkey();
        assert!(txn_pool.txns.len() == 0);
    }
    #[test]
    fn add_txn_to_pool() -> Result<()> {
        use super::*;

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
}
