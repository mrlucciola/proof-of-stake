use posbc::ledger::{
    blockchain::Blockchain,
    blocks::{Block, BlockTxnMap},
    txn::TxnId,
};

use super::UserInfo;

/// Creates an empty block using the leader and previous block.
///
/// Does not populate with transactions.
pub fn create_block_from_last(leader: UserInfo, prev_block: &Block) -> Block {
    let prev_block_id: TxnId = prev_block.id();
    let prev_blockheight = prev_block.blockheight;
    let leader = leader.pbkey();

    Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight)
}
/// Creates an empty block using the leader and previous block.
///
/// Similar function to `create_block()` but gets last block automatically from the block-chain.\
/// Does not populate with transactions.
pub fn create_block(leader: UserInfo, blockchain: &Blockchain) -> Block {
    // assuming blockchain was created properly
    let prev_block = get_last_block(blockchain).expect("Error getting last block in blockchain.");

    create_block_from_last(leader, prev_block)
}

/// Get most recent block of the given blockchain.
///
/// @todo make this a getter method on `Blockchain`
fn get_last_block(blockchain: &Blockchain) -> Option<&Block> {
    blockchain.blocks().values().next_back()
}
