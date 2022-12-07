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
        main,
        send: _,
        recv: _,
    } = init_users();
    let blockchain = Blockchain::new();

    let blocks = blockchain.blocks();

    // blockchain must have length = 1
    assert!(blocks.len() == 1, "Blockchain must have one block");

    // all blocks must be valid (no `None` fields, correct hash, signed)
    let mut genesis = blocks.values().to_owned().next().unwrap().to_owned();

    genesis.sign(&main.wallet);

    let is_valid = genesis.is_valid(&main.wallet).unwrap();
    assert!(is_valid, "Invalid genesis block: {:?}", is_valid);

    // block must be genesis
    assert!(
        Blockchain::is_genesis_block(&genesis),
        "Not genesis block: {:?}",
        genesis.id()
    );
}
