// external
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
// local
pub use crate::ledger::{
    block::{block_id::BlockId, block_signature::BlockSignature, types::*},
    general::{PbKey, Result},
};

/// ## Block header
///
/// Data contained within a block header.
///
/// @todo this should not hold transactions - temporary, for serialization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// List/map of all transactions to be included in the block
    txns: BlockTxnMap,
    /// Public key of the current block proposer (node)
    leader: PbKey,
    /// Identifier of the previous block - hash digest
    prev_block_id: BlockId,
    /// Block height - current number of blocks in blockchain + 1
    blockheight: u128,
    /// Current time - unix time stamp
    system_time: u64,
}

// Add constructor and getters
impl BlockHeader {
    pub fn new(
        txns: BlockTxnMap,
        leader: PbKey,
        prev_block_id: BlockId,
        prev_blockheight: u128,
    ) -> Self {
        // calculate the blockheight
        let blockheight = prev_blockheight + 1;

        Self {
            txns,
            leader,
            prev_block_id,
            blockheight,
            system_time: Utc::now().timestamp_millis().try_into().unwrap(),
        }
    }
    pub fn genesis(txns: BlockTxnMap, leader: PbKey) -> Self {
        Self::new(txns, leader, BlockId::from_bytes([0u8; 64]), 0)
    }

    /// ### Get property `Block.txns`.
    pub fn txns(&self) -> &BlockTxnMap {
        &self.txns
    }
    /// ### Get property `Block.txns` as mutable.
    pub fn txns_mut(&mut self) -> &mut BlockTxnMap {
        self.txns.borrow_mut()
    }
    /// ### Get property `Block.leader`.
    pub fn leader(&self) -> &PbKey {
        &self.leader
    }
    /// ### Get property `Block.prev_block_id`.
    pub fn prev_block_id(&self) -> &BlockId {
        &self.prev_block_id
    }
    /// ### Get property `Block.blockheight`.
    pub fn blockheight(&self) -> &u128 {
        &self.blockheight
    }

    /// ### Convert to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate if possible
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(self).expect("Error serializing block header")
    }
}
