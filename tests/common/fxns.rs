use posbc::{
    ledger::{
        blocks::{Block, BlockTxnMap},
        txn::TxnId,
    },
    utils::hash::BlakeHash,
};

use super::UserInfo;

/// Creates an empty
pub fn create_block(leader: UserInfo) -> Block {
    let prev_block_id: TxnId = BlakeHash::from_bytes([0u8; 32]);
    let prev_blockheight = 0;
    let leader = leader.pbkey();

    Block::new(BlockTxnMap::new(), leader, prev_block_id, prev_blockheight)
}
