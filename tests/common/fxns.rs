use posbc::{
    accounts::{
        account::Account,
        accounts::{AccountMap, Accounts},
    },
    ledger::{
        block::{block_id::BlockId, types::BlockTxnMap, Block},
        blockchain::Blockchain,
    },
};
// test
use super::{init_users, UserInfo, UsersInfo};

/// ### Creates an empty block using the leader and previous block.
/// Does not populate with transactions.
pub fn create_block_from_last(leader: &UserInfo, prev_block: &Block) -> Block {
    let prev_block_id: BlockId = prev_block.id();
    let prev_blockheight = prev_block.blockheight();
    let leader = leader.pbkey();

    Block::new(
        BlockTxnMap::new(),
        leader,
        prev_block_id,
        prev_blockheight.to_owned(),
    )
}

/// ### Creates an empty block using the leader and previous block.
/// Similar function to `create_block()` but gets last block automatically from the block-chain.\
/// Does not populate with transactions.
pub fn create_block(leader: &UserInfo, blockchain: &Blockchain) -> Block {
    // assuming blockchain was created properly
    let prev_block = get_last_block(blockchain).expect("Error getting last block in blockchain.");

    create_block_from_last(leader, prev_block)
}

/// ### Get most recent block of the given blockchain.
///
/// @todo make this a getter method on `Blockchain`
fn get_last_block(blockchain: &Blockchain) -> Option<&Block> {
    blockchain.blocks().values().next_back()
}

/// ### Initialize account map.
///
/// Add sender account as first account to the chain.\
/// Sender account has balance of 1000.
pub fn init_account_map(blockchain: &mut Blockchain) {
    // init
    let users = init_users();
    let send_acct = Account::new(&users.send.pbkey().into(), Some(1000));

    blockchain.accounts_mut().add_acct(send_acct);
}
pub fn init_blockchain() -> (UsersInfo, Blockchain) {
    let users = init_users();
    let blockchain = Blockchain::new();

    (users, blockchain)
}
pub fn init_blockchain_and_accounts() -> (UsersInfo, Blockchain) {
    let (users, mut blockchain) = init_blockchain();
    init_account_map(&mut blockchain);

    (users, blockchain)
}
/// ### Get first account in account map.
pub fn get_first_acct(accounts: &Accounts) -> Option<&Account> {
    let accts: &AccountMap = accounts.accounts();

    accts.values().next()
}
