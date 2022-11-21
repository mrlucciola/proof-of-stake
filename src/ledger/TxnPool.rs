use super::txn::Txn;

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
}

mod tests {
    use crate::ledger::{txn::TxnType, wallet::Wallet};
    use super::*;

    #[test]
    fn create_txn_pool() {
        use super::*;

        let mut txn_pool = TxnPool::new();
        let pbkey1 = Wallet::new_from_file(&"./test_key.json".to_string()).get_pbkey();
        let pbkey2 = Wallet::new_from_file(&"./test_key_recv.json".to_string()).get_pbkey();
        let txn_1 = Txn::new(pbkey1, pbkey2, 100, TxnType::Transfer);
        txn_pool.txns.push(txn_1);

        assert!(txn_pool.txns.len() == 1)
    }
}
