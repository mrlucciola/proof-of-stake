// imports
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
// local
use super::{
    blocks::{Block, BlockTxnMap},
    general::PbKey,
    wallet::Wallet,
};

/// Lookup type for the `blocks` map a string
pub type BlockMapKey = String; // TODO: change to hex
pub type BlockMap = BTreeMap<BlockMapKey, Block>;
/// Data structure, contains list of sequential block
#[derive(Debug, Deserialize, Serialize)]
pub struct Blockchain {
    blocks: BlockMap,
}
impl Blockchain {
    /// Create new `Blockchain` instance
    ///
    /// Contains list (BTreeMap) of blocks in sequence, queriable by their ID, in string form
    ///
    /// The first block in a blockchain is the genesis block
    pub fn new() -> Self {
        let blocks = BlockMap::new();

        // Create the genesis block
        // TODO: move this to a separate private method on `Block`
        let leader_wallet = Wallet::new_from_file(&"hidden/master_key.json".to_string());
        let leader: PbKey = leader_wallet.pbkey();

        let mut genesis_block = Block::new(BlockTxnMap::new(), leader, [0u8; 32], 0);
        genesis_block.blockheight = 0;
        genesis_block.set_id();

        let mut blockchain = Self { blocks };

        blockchain.add_block(genesis_block);

        blockchain
    }
    /// Add a new block to the blockchain
    pub fn add_block(&mut self, block: Block) -> &mut Block {
        // check if block is valid
        // check if block is signed
        // check if entry exists -> if not, then insert
        self.blocks.entry(block.id_key()).or_insert(block)
    }
    pub fn blocks(&self) -> &BlockMap {
        &self.blocks
    }
}
