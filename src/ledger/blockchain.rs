// imports
use serde::Serialize;
use std::collections::BTreeMap;
// local
use super::{
    blocks::{Block, BlockId, BlockTxnMap},
    general::PbKey,
    wallet::Wallet,
};

/// Lookup type for the `blocks` map a string
pub type BlockMapKey = String; // TODO: change to hex
pub type BlockMap = BTreeMap<BlockMapKey, Block>;
/// Data structure, contains list of sequential block
#[derive(Debug, Serialize)]
pub struct Blockchain {
    blocks: BlockMap,
}
impl Blockchain {
    /// Create new `Blockchain` instance
    ///
    /// Contains list (BTreeMap) of blocks in sequence, queriable by their ID, in string form
    ///
    /// The first block in a blockchain is the genesis block.
    pub fn new() -> Self {
        let blocks = BlockMap::new();
        let mut blockchain = Self { blocks };

        // Create the genesis block
        blockchain.genesis();

        blockchain
    }
    /// Add a new block to the blockchain.
    pub fn add_block(&mut self, block: Block) -> &mut Block {
        // check if block is valid
        // check if block is signed
        // check if entry exists -> if not, then insert
        self.blocks.entry(block.id_key()).or_insert(block)
    }
    /// Getter for `blocks`
    pub fn blocks(&self) -> &BlockMap {
        &self.blocks
    }
    /// Create and add the genesis block.
    ///
    /// The genesis block is the initial/seed block for the entire blockchain.
    fn genesis(&mut self) {
        if !self.blocks().is_empty() {
            panic!("Blockchain needs to be empty")
        }

        let leader_wallet = Wallet::new_from_file(&"hidden/master_key.json".to_string());
        let leader: PbKey = leader_wallet.pbkey();

        // creates a new block using the `Block` constructor - we need to replace the blockheight, id, and signature
        let mut genesis_block = Block::new(
            BlockTxnMap::new(),
            leader,
            BlockId::from_bytes([0u8; 32]),
            0,
        );
        // replace blockheight
        genesis_block.blockheight = 0;
        // replace id/hash
        genesis_block.set_id();

        self.add_block(genesis_block);
    }
    pub fn is_genesis_block(block: &Block) -> bool {
        let block_id = Block::calc_id(block);
        block.id() == [0u8; 32]
    }
    /// Check if the current blockheight is valid
    pub fn is_blockheight_valid(&self) -> bool {
        self.blocks.values().last().unwrap().blockheight
            == (self.blocks().len() - 1).try_into().unwrap()
    }
}
