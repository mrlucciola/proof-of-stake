// imports
// local
use posbc::ledger::{
    blocks::{Block, BlockTxnMap},
    txn::{Txn, TxnHash, TxnType},
};

mod common;
use common::{init_users, UsersInfo};

fn create_sample_txn_map(num_txns: u8) -> BlockTxnMap {
    let UsersInfo {
        main: _,
        send,
        recv,
    } = init_users();
    let mut txn_map = BlockTxnMap::new();

    if num_txns == 0 {
        return txn_map;
    }

    // logic to create sample txn > add txn to map
    for x in 0..num_txns {
        let amt = (x as u128) + 1;
        // create sample txn
        let new_txn = Txn::new_signed(&send.wallet, recv.kp.public_key(), amt, TxnType::Transfer);

        // add txn to map
        txn_map.insert(new_txn.hash(), new_txn);
    }

    txn_map
}

#[test]
fn create_empty_block() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();

    // create txn map
    let txn_map: BlockTxnMap = create_sample_txn_map(0);
    let genesis_blockhash: TxnHash = [0u8; 32];

    // check if hashes line up
    let new_block = Block::new(txn_map, main.kp.public_key(), genesis_blockhash, 0);
    assert_eq!(new_block.hash(), new_block.hash, "{:?}", new_block.hash());
    assert_eq!(new_block.hash(), "".as_bytes(), "{:?}", new_block.hash());
}
#[test]
fn create_full_block() {
    let UsersInfo {
        main,
        send: _,
        recv: _,
    } = init_users();

    // create txn map
    let txn_map: BlockTxnMap = create_sample_txn_map(3);
    let genesis_blockhash: TxnHash = [0u8; 32];

    let new_block = Block::new(txn_map, main.kp.public_key(), genesis_blockhash, 0);

    // check if hashes line up
    assert_eq!(new_block.hash(), new_block.hash, "{:?}", new_block.hash());
    assert_eq!(new_block.hash(), "".as_bytes(), "{:?}", new_block.hash());
}
