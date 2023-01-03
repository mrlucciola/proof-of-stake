// imports
// local
use posbc::ledger::blockchain::Blockchain;

pub mod accounts;
pub mod common;
use common::{fxns::create_block, init_users, UsersInfo};

// tests:

fn init_blockchain() -> (UsersInfo, Blockchain) {
    let users = init_users();
    let blockchain = Blockchain::new();

    (users, blockchain)
}

#[test]
fn create_blockchain_pass() {
    let (users, blockchain) = init_blockchain();

    let blocks = blockchain.blocks();

    // blockchain must have length = 1
    assert!(blocks.len() == 1, "Blockchain must have one block");

    // all blocks must be valid (no `None` fields, correct hash, signed)
    let mut genesis = blocks.values().to_owned().next().unwrap().to_owned();

    genesis.sign(&users.main.wallet);

    let is_valid = genesis.is_valid(&users.main.wallet).unwrap();
    assert!(is_valid, "Invalid genesis block: {:?}", is_valid);

    // block must be genesis
    assert!(
        Blockchain::is_genesis_block(&genesis),
        "Not genesis block: {:?}",
        genesis.id()
    );
}

/// add a block to the blockchain
/// Check if the block map length increased by 1
/// Check if that new block exists
#[test]
fn add_block_to_blockchain_pass() {
    let (users, mut blockchain) = init_blockchain();

    let blocks_len_pre = blockchain.blocks().len();

    // add a block to the chain
    let new_block_to_add = create_block(users.main);
    let key = new_block_to_add.id_key();
    blockchain.add_block(new_block_to_add);
    let blocks_len_post = blockchain.blocks().len();

    // check
    assert_eq!(
        blocks_len_post,
        blocks_len_pre + 1,
        "Block count is incorrect"
    );

    // see if its the same block
    assert!(
        blockchain.block(&key).is_some(),
        "Block is not in blockchain"
    );
}
