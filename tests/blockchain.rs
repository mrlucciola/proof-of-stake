// imports
// local
use posbc::ledger::{
    blockchain::{Blockchain, BlocksMap},
    blocks::{Block, BlockTxnMap},
    txn::{Txn, TxnHash, TxnType},
};

pub mod common;
use common::{init_users, UsersInfo};

// tests:

#[test]
fn create_blockchain_pass() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();
    let blockchain = Blockchain::new();
    
}
