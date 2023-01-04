// imports
// local
use posbc::{
    accounts::account::Account,
    ledger::{blockchain::Blockchain, general::Result, txn::Txn},
};

pub mod accounts;
pub mod common;
use common::{
    fxns::{create_block, get_first_acct, init_account_map},
    init_users, UsersInfo,
};

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

/// Add a block to the blockchain.
///
/// Check if the block map length increased by 1.\
/// Check if that new block exists.
#[test]
fn add_block_to_blockchain_pass() {
    let (users, mut blockchain) = init_blockchain();

    let blocks_len_pre = blockchain.blocks().len();

    // add a block to the chain
    let new_block_to_add = create_block(users.main, &blockchain);
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
    use posbc::ledger::txn::TxnType;
    // init
    let (users, mut blockchain) = init_blockchain();

    let amt_to_send = 1;

    init_account_map(&mut blockchain);
    assert!(
        blockchain.accounts.accounts().len() == 1,
        "length: {}",
        blockchain.account_map().len()
    );

    let bal_send_pre = blockchain
        .accounts
        .acct_balance(&users.send.pbkey().to_string());
    let bal_recv_pre = blockchain
        .accounts
        .acct_balance(&users.recv.pbkey().to_string());

    // populate the block with a transaction
    let txn_to_add = Txn::new_signed(
        &users.send.wallet,
        users.recv.pbkey(),
        amt_to_send,
        TxnType::Transfer,
    );
    blockchain.process_transfer_txn(&txn_to_add)?;

    // Verify account length increased to 2
    assert!(
        blockchain.accounts.accounts().len() == 2,
        "length: {}",
        blockchain.account_map().len()
    );

    let bal_send_post = blockchain
        .accounts
        .acct_balance(&users.send.pbkey().to_string());
    let bal_recv_post = blockchain
        .accounts
        .acct_balance(&users.recv.pbkey().to_string());
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
