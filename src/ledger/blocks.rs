// imports
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_big_array::{self, BigArray};
use std::collections::HashMap;
// local
use crate::ledger::{
    general::PbKey,
    txn::{Txn, TxnHash},
};

// export types
pub type BlockHash = [u8; 32];
pub type BlockSignature = [u8; 64];
// TODO: add condition that this map cant have more than _ number of txns
pub type BlockTxnMap = HashMap<TxnHash, Txn>;

/// Info contained within a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// list/map of all transactions to be included in the block
    pub transactions: BlockTxnMap,
    /// public key of the current block proposer (node)
    pub leader: PbKey,
    pub prev_blockhash: BlockHash,
    /// block height - current number of blocks in blockchain + 1
    pub block_height: u128,
    /// current time - unix time stamp
    pub system_time: u64,
    /// hash of the current block
    pub hash: BlockHash,
    /// the leader's signature for this block submission - Ecdsa signature
    #[serde(with = "BigArray")]
    pub signature: BlockSignature,
}

impl Block {
    /// `Block` constructor fxn - create a new block (not genesis block).
    ///
    /// transactions: List of transactions (`Txn`) to be included in the block\
    /// TODO: add `blockchain` as param - use it to get block count
    pub fn new(
        transactions: BlockTxnMap,
        leader: PbKey,
        prev_blockhash: BlockHash,
        prev_block_height: u128,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();
        let block_height = prev_block_height + 1;

        let mut block = Self {
            transactions,
            leader,
            prev_blockhash,
            block_height,
            system_time,
            hash: [0u8; 32],
            signature: [0u8; 64],
        };

        // set the hash with the body
        block.set_hash();
        // return the block
        block
    }
    /// Get and set the `hash` for `block` object.
    ///
    /// Returns hash
    fn set_hash(&mut self) -> BlockHash {
        let hash = self.hash();
        self.hash = hash;

        hash
    }
    /// Method wrapper/analog for `get_hash()`
    pub fn hash(&self) -> BlockHash {
        Self::get_hash(&self)
    }
    /// Compute the hash digest of the block - associated fxn
    ///
    /// Zero-out the has and signature in order to compute properly
    pub fn get_hash(block: &Block) -> BlockHash {
        let mut adj_block_body = block.clone();
        // set blank vars
        adj_block_body.hash = [0u8; 32];
        adj_block_body.signature = [0u8; 64];

        // serialize to a byte vector
        let block_msg_bytes: Vec<u8> = serde_json::to_vec(&adj_block_body).unwrap();

        // get hash digest of block
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"block-v0");
        hasher.update(&block_msg_bytes);
        let hash: BlockHash = hasher.finalize().as_bytes().to_owned();

        hash
    }
}
