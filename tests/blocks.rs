use std::borrow::Borrow;

// imports
// local
use posbc::ledger::{
    blocks::{Block, BlockTxnMap},
    txn::{Txn, TxnHash, TxnType},
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
fn create_empty_block() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();

    // genesis
    let prev_blockhash: TxnHash = [0u8; 32];
    let prev_block_height = 0;
    let leader = main.pbkey();
    let mut block = Block::new(
        BlockTxnMap::new(),
        leader,
        prev_blockhash,
        prev_block_height,
    );
    // create txn map
    add_sample_txns_to_block(0, &mut block);

    // check if hashes line up
    assert_eq!(block.hash(), block.hash, "{:?}", block.hash());
}

#[test]
fn create_full_block() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();

    // genesis
    let prev_blockhash: TxnHash = [0u8; 32];
    let prev_block_height = 0;
    let leader = main.pbkey();
    let mut block = Block::new(
        BlockTxnMap::new(),
        leader,
        prev_blockhash,
        prev_block_height,
    );
    // create txn map
    add_sample_txns_to_block(3, &mut block);

    // for tx in block.transactions.iter() {
    //     println!("\n\ntx: {:?}--\n{:?}\n", tx.0, tx.1);
    // }

    // check if hashes line up
    assert_eq!(block.hash(), block.hash, "{:?}", block.hash());
    // assert_eq!(new_block.hash(), "".as_bytes(), "{:?}", new_block.hash());
}
