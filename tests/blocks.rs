// imports
// local
use posbc::{
    ledger::{
        blocks::{Block, BlockTxnMap},
        txn::{Txn, TxnId, TxnType},
    },
    utils::hash::BlakeHash,
};

pub mod common;
use common::{init_users, UsersInfo};

// tests:
// create a block
// add txn to block
// add block to chain

fn add_sample_txns_to_block(num_txns: u8, block: &mut Block) {
    let UsersInfo {
        main: _,
        send,
        recv,
    } = init_users();
    // logic to create sample txn > add txn to map
    for x in 0..num_txns {
        let amt_to_send = (x as u128) + 1;
        // create sample txn
        let new_txn = Txn::new_signed(&send.wallet, recv.pbkey(), amt_to_send, TxnType::Transfer);

        // add txn to map
        block.add_txn(new_txn);
    }
}

#[test]
fn create_empty_block_pass() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();

    // genesis
    let prev_block_id: TxnId = BlakeHash::from_bytes([0u8; 32]);
    let prev_blockheight = 0;
    let leader = main.pbkey();
    let mut block = Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight);
    // create txn map
    add_sample_txns_to_block(0, &mut block);

    // check if hashes line up
    assert_eq!(block.id(), block.calc_id(), "{:?}", block.id());
}

#[test]
fn create_full_block_pass() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();

    // genesis
    let prev_block_id: TxnId = BlakeHash::from_bytes([0u8; 32]);
    let prev_blockheight = 0;
    let leader = main.pbkey();
    let mut block = Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight);
    // create txn map
    add_sample_txns_to_block(3, &mut block);

    // check if hashes line up
    assert_eq!(block.id(), block.calc_id(), "{:?}", block.id());
}
