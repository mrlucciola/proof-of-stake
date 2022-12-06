// imports
// local
use posbc::ledger::blockchain::Blockchain;

pub mod accounts;
pub mod common;
use common::{init_users, UsersInfo};

// tests:

#[test]
fn create_blockchain_pass() {
    let UsersInfo {
        main: _,
        send: _,
        recv: _,
    } = init_users();
    let blockchain = Blockchain::new();

    let blocks = blockchain.blocks();

    // blockchain must have length = 1
    assert!(blocks.len() == 1, "Blockchain must have one block");

    // all blocks must be valid (no `None` fields, correct hash, signed)
    let genesis = blocks.values().next().unwrap();
    // assert!(genesis.is_valid());

    // block must be genesis
    // assert!(genesis.is_genesis());
}
