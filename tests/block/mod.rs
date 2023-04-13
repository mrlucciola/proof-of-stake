// local
use posbc::ledger::{
    block::{types::BlockTxnMap, Block, BlockId, BlockSignature},
    txn::{Txn, TxnType},
};
// test
use crate::common::{init_users, UsersInfo};

// tests:
// create a block
// add txn to block
// add block to chain

fn add_sample_txns_to_block(num_txns: u8, block: &mut Block) {
    let UsersInfo {
        main: _,
        send,
        recv,
        test1: _,
        test2: _,
        test3: _,
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
        test1: _,
        test2: _,
        test3: _,
    } = init_users();

    // genesis
    let prev_block_id = BlockId::from_bytes([0u8; 64]);
    let prev_blockheight = 0;
    let leader = main.pbkey();
    let mut block = Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight);
    // create txn map
    add_sample_txns_to_block(0, &mut block);

    // check if hashes line up
    assert_eq!(
        block.id(),
        BlockId::from(block.calc_id()),
        "{:?}",
        block.id()
    );
}

#[test]
fn create_full_block_pass() {
    let users: UsersInfo = init_users();
    let main = users.main;

    // genesis
    let prev_block_id = BlockId::from_bytes([0u8; 64]);
    let prev_blockheight = 0;
    let leader = main.pbkey();
    let mut block = Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight);
    block.system_time = 1681353445765;
    block.set_id();
    let block_id_test = BlockId::from(block.calc_id());
    let block_id_answer_bytestr = b"6d7961d34512bac3f5b6d4e09074057053d30d0e2f4c7ab5e6fabcced4d1ece0eaee5cae92743f8d11c899db4201cf3d06fe0c3792f67b2fad1e1f1a332fe101";
    let block_id_answer: [u8; 64] = hex::decode(block_id_answer_bytestr)
        .unwrap()
        .try_into()
        .unwrap();
    let block_id_answer: BlockId = BlockId::from_bytes(block_id_answer);
    assert_eq!(block_id_test, block_id_answer, "{block_id_answer:?}");

    // sign
    block.sign(&main.wallet);
    let sig_answer_bytestr1 = b"5f7036771eae4ec782cf57433437825a5624f1305bba495d097de3a7a83b602f723e67e595f945bc439565f2091aeb41381d04bd3f33423a73efe0207d369c05";
    let sig_answer1: [u8; 64] =
        ed25519_dalek::Signature::from_bytes(&hex::decode(sig_answer_bytestr1).unwrap())
            .unwrap()
            .to_bytes();
    assert_eq!(
        block.signature().to_str(),
        BlockSignature(sig_answer1.to_vec()).to_str(),
        "{sig_answer1:?}"
    );

    // create txn map
    add_sample_txns_to_block(3, &mut block);
    block.sign(&main.wallet);
    // should return a different signature
    assert_ne!(
        block.signature().to_str(),
        BlockSignature(sig_answer1.to_vec()).to_str(),
        "{sig_answer1:?}"
    );
}
#[test]
fn is_signature_valid_pass() {
    let users: UsersInfo = init_users();
    let main = users.main;

    let prev_block_id = BlockId::from_bytes([0u8; 64]);
    let prev_blockheight = 0;
    let leader = main.pbkey();
    let mut block = Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight);
    block.sign(&main.wallet);

    assert_eq!(
        block.is_signature_valid(&main.wallet.pbkey()).unwrap(),
        (),
        "{block:?}",
    );

    block.sign(&main.wallet);
    assert_eq!(
        block.is_signature_valid(&main.wallet.pbkey()).unwrap(),
        (),
        "{block:?}",
    );
}
