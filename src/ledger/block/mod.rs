pub mod block_id;
pub mod block_signature;
pub mod constants;
mod error;
mod getters;
mod setters;
pub mod types;
mod utils;
mod validation;
// external
use {chrono::prelude::*, serde::Serialize};
// local
use crate::ledger::general::{PbKey, Result};
pub use {block_id::BlockId, block_signature::BlockSignature, types::*};

/// ### Info contained within a block
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    /// List/map of all transactions to be included in the block
    txns: BlockTxnMap,
    /// Public key of the current block proposer (node)
    leader: PbKey,
    /// Identifier of the previous block - hash digest
    pub prev_block_id: BlockId,
    /// Block height - current number of blocks in blockchain + 1
    blockheight: u128,
    /// Current time - unix time stamp
    pub system_time: u64,
    /// Identifier/ID - hash digest of the current block
    #[serde(skip_serializing)]
    pub id: Option<BlockId>,
    /// The leader's signature for this block submission - Ecdsa signature
    #[serde(skip_serializing)]
    pub signature: Option<BlockSignature>,
}

impl Block {
    /// ### `Block` constructor fxn - create a new unsigned block (not genesis block).
    /// transactions: List of transactions (`Txn`) to be included in the block\
    /// TODO: add `blockchain` as param - use it to get block count
    /// @todo allow `None` input for `txns` to default to a new block txn map
    pub fn new(
        txns: BlockTxnMap,
        leader: PbKey,
        prev_block_id: BlockId,
        prev_blockheight: u128,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();
        let blockheight = prev_blockheight + 1;

        let mut block = Self {
            txns,
            leader: leader.into(),
            prev_block_id,
            blockheight,
            system_time,
            id: None,
            signature: None,
        };

        // set the id (hash) with the body
        block.set_id();

        block
    }
    /// ## Create and add the genesis block.
    ///
    /// The genesis block is the initial/seed block for the entire blockchain.
    ///
    /// Notes:
    /// - Validates that no prior blocks exist.
    /// - Manually assigns a blocktime (0 = 1/1/1970 00:00).
    pub fn new_genesis(initializer: PbKey) -> Result<Block> {
        // create a new block using the `Block` constructor - we need to replace the blockheight, id, and signature
        let mut genesis_block = Block::new(
            BlockTxnMap::new(),
            initializer,
            BlockId::from_bytes([0u8; 64]),
            0,
        );
        // replace blockheight & time
        genesis_block.blockheight = 0;
        genesis_block.system_time = 0;
        // replace id/hash
        genesis_block.set_id(); // make private
        Ok(genesis_block)
    }
}
