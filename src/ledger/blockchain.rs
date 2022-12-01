use secp256k1::Secp256k1;
// imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// local
use super::{
    blocks::{Block, BlockTxnMap},
    general::{PbKey, PvKey},
};

pub type BlocksMap = HashMap<String, Block>;

/// Data structure, contains list of sequential block
#[derive(Debug, Deserialize, Serialize)]
struct Blockchain {
    blocks: HashMap<String, Block>,
}
impl Blockchain {
    /// Create new `Blockchain` instance
    ///
    /// Contains list (hashmap) of blocks in sequence, queriable by their ID, in string form
    ///
    /// The first block in a blockchain is the genesis block
    pub fn new() -> Self {
        let mut blocks = BlocksMap::new();

        // Create the genesis block
        // TODO: move this to a separate private method
        let leader: PbKey = PvKey::from_slice(&[0u8; 32])
            .expect("32 bytes, within curve order")
            .public_key(&Secp256k1::new());
        let mut genesis_block = Block::new(BlockTxnMap::new(), leader, [0u8; 32], 0);
        genesis_block.block_height = 0;
        genesis_block.set_hash();

        blocks.insert(genesis_block.hash_str(), genesis_block);
        Self { blocks }
    }
}
