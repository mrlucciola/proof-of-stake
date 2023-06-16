pub mod block_header;
pub mod block_id;
pub mod block_signature;
pub mod constants;
mod error;
mod getters;
pub mod setters;
pub mod types;
mod utils;
mod validation;
// external
use serde::{Deserialize, Serialize};
// local
use crate::ledger::{
    block::{
        block_header::BlockHeader, block_id::BlockId, block_signature::BlockSignature, types::*,
    },
    general::{PbKey, Result},
};

/// ## Info contained within a block
///
/// @todo create a `BlockHeader` struct to hold all fields except `txns` and `signature`;\
/// @todo create method to check if block is over gas limit (create gas value for each `Txn` type);
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    header: BlockHeader,
    /// Identifier/ID - hash digest of the current block
    id: Option<BlockId>,
    /// The leader's signature for this block submission - Ecdsa signature
    signature: Option<BlockSignature>,
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
        // create block header
        let header = BlockHeader::new(txns, leader, prev_block_id, prev_blockheight);
        let mut block = Self {
            header,
            id: None,
            signature: None,
        };
        // set the id (hash) with the body
        block.update_id();

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
        // create genesis block header
        let genesis_block = BlockHeader::genesis(BlockTxnMap::new(), initializer);
        // create a new block using the `Block` constructor - we need to replace the blockheight, id, and signature
        let mut genesis_block = Self {
            header: genesis_block,
            id: None,
            signature: None,
        };
        // set id/hash
        genesis_block.update_id();

        Ok(genesis_block)
    }

    /////////////////////////////////////////////////
    //////////////// PRIVATE SETTERS ////////////////
    /// ### Calculate and set the id for a `Block`.
    /// Returns id.
    fn update_id(&mut self) -> BlockId {
        let new_id = self.calc_id();
        self.id = Some(new_id);

        new_id
    }
    /// ### Set the signature for the block.
    fn set_signature(&mut self, signature: BlockSignature) {
        self.signature = Some(signature);
    }
    //////////////// PRIVATE SETTERS ////////////////
    /////////////////////////////////////////////////
}
