use posbc::ledger::{
    block::{types::BlockTxnMap, Block},
    general::Result,
    txn::{Txn, TxnType},
    txn_pool::{TxnMap, TxnPool},
};
use std::{thread, time};
// test
use crate::common::fxns::{create_block, init_blockchain, init_blockchain_and_accounts};

#[test]
fn create_blockchain_pass() {
    let (users, mut blockchain) = init_blockchain();
    let main = users.main;
    let genesis = Block::new_genesis(&main.wallet);

    blockchain
        .add_block(genesis)
        .expect("Error adding genesis block to blockchain.");

    let blocks = blockchain.blocks();

    // blockchain must have length = 1
    assert!(blocks.len() == 1, "Blockchain must have one block");

    // all blocks must be valid (no `None` fields, correct hash, signed)
    let genesis = blocks.values().next().unwrap().to_owned();

    genesis.calc_signature(&main.wallet);

    let is_valid = genesis.is_valid(&main.wallet.pbkey()).is_ok();
    assert!(is_valid, "Invalid genesis block: {:?}", is_valid);

    // block must be genesis
    assert!(
        blockchain.is_genesis_block(&genesis),
        "Not genesis block: {:?}",
        genesis.id()
    );
}

/// Add a block to the blockchain.
///
/// Check if the block map length increased by 1.\
/// Check if that new block exists.
#[test]
fn add_block_to_blockchain_pass() {
    let (users, mut blockchain) = init_blockchain();
    let main = users.main;

    let blocks_len_init = blockchain.blocks().len();
    assert_eq!(
        blocks_len_init, 0,
        "Blockchain must have 0 blocks after initializing"
    );

    let genesis = Block::new_genesis(&main.wallet);
    blockchain
        .add_block(genesis)
        .expect("Error adding genesis block to blockchain.");

    let blocks_len_pre = blockchain.blocks().len();
    assert_eq!(
        blocks_len_pre,
        blocks_len_init + 1,
        "Blockchain must have 1 block after adding genesis"
    );

    // add a block to the chain
    let mut new_block_to_add = create_block(&main, &blockchain);

    new_block_to_add.sign(&main.wallet);
    let key = new_block_to_add.id_key();
    blockchain
        .add_block(new_block_to_add)
        .expect("Error adding second block to blockchain");
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

/// Execute a single state update defined by the transaction.
///
/// Full flow includes:
///   1. Taking txns from mempool and grouping into prospective block
///   1. Execute txns/state updates
///   1. Handle failure cases
///
/// In this test, recv account should not exist at the start, should be created during the process transfer txn call
/// recv ending balance should be 1 (amt to send)
/// send ending balance should be 999=1000-1 (init - amt to send)
#[test]
fn execute_txn_via_blockchain_pass() -> Result<()> {
    // init
    let (users, mut blockchain) = init_blockchain_and_accounts();

    let amt_to_send = 1;
    assert!(
        blockchain.accounts().accounts().len() == 1,
        "incorrect length: {}",
        blockchain.account_map().len()
    );

    let bal_send_pre = blockchain
        .accounts()
        .acct_balance(&users.send.pbkey().into());
    let bal_recv_pre = blockchain
        .accounts()
        .acct_balance(&users.recv.pbkey().into());

    // populate the block with a transaction
    let txn_to_add = Txn::new_signed(
        &users.send.wallet,
        users.recv.pbkey(),
        amt_to_send,
        TxnType::Transfer,
    );
    blockchain.add_txn_to_block(&txn_to_add)?;

    // Verify account length increased to 2
    assert!(
        blockchain.accounts().len() == 2,
        "length: {}",
        blockchain.account_map().len()
    );

    let bal_send_post = blockchain
        .accounts()
        .acct_balance(&users.send.pbkey().into());
    let bal_recv_post = blockchain
        .accounts()
        .acct_balance(&users.recv.pbkey().into());
    assert_eq!(
        bal_send_pre + bal_recv_pre,
        bal_send_post + bal_recv_post,
        "Starting and ending balances are not equal"
    );
    assert_eq!(
        bal_send_pre - bal_send_post,
        amt_to_send,
        "Send final balance did not decrease by correct amount."
    );
    assert_eq!(
        bal_recv_post - bal_recv_pre,
        amt_to_send,
        "Recv final balance did not increase by correct amount."
    );

    Ok(())
}

#[test]
fn add_txn_to_blocks_pass() -> Result<()> {
    let (users, mut blockchain) = init_blockchain_and_accounts();
    let main = users.main;
    let send = users.send;
    let recv = users.recv;

    let genesis = Block::new_genesis(&main.wallet);
    blockchain
        .add_block(genesis)
        .expect("Error adding genesis block to blockchain.");

    // create txn pool
    let mut txn_pool = TxnPool::new();
    // create txn map
    let mut temp_txn_map = TxnMap::new();

    // create txns and add them to the map
    let ten_millis = time::Duration::from_millis(10);

    // txn config
    let amt_to_send = 1;
    let txn_type = TxnType::Transfer;
    let txn_ct = 5;

    for _ in 0..txn_ct {
        thread::sleep(ten_millis);
        let txn: Txn = Txn::new_signed(&send.wallet, recv.pbkey(), amt_to_send, txn_type);

        txn_pool.add_txn(txn.clone())?;
        // @todo only add txn id, not whole txn
        if let Some(_) = temp_txn_map.insert(txn.clone().id_key(), txn.clone()) {
            panic!("Txn already exists in temp map.");
        }
    }
    assert!(txn_pool.txn_ct() == 5, "Txn pool isnt 5");
    assert!(temp_txn_map.len() == 5, "Txn map isnt 5");

    // add txns to block as they are processed
    // get the genesis block
    let prev_block = blockchain.last_block();
    let mut new_block = Block::new(
        BlockTxnMap::new(),
        main.pbkey(),
        prev_block.id(),
        prev_block.blockheight().to_owned(),
    );

    // PROCESS ALL TRANSACTIONS
    blockchain.add_txn_to_blocks(&temp_txn_map, &mut new_block, &mut txn_pool)?;

    // assert that the block has all of the txns to be added
    assert!(
        new_block.txns().len() == txn_ct,
        "Block has incorrect number of txns. Should have {}, currently has {} transactions.",
        txn_ct,
        new_block.txns().len()
    );
    assert!(
        txn_pool.txn_ct() == 0,
        "Transaction pool has incorrect number of txns. Should have 0, currently has {} transactions.",
        txn_pool.txn_ct()
    );

    // assert that the txn IDs match between the block txn and temp txn maps
    for (_key, txn) in temp_txn_map.iter() {
        assert!(!new_block.txns().get(&txn.id_key()).is_none());
    }

    Ok(())
}
