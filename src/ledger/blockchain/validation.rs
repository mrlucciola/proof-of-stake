use crate::ledger::block::{Block, BlockDigest, BlockId};

use super::Blockchain;

impl Blockchain {
    /// Check if the current blockheight is valid
    /// @todo call this after block genesis creation
    pub fn is_blockheight_valid(&self) -> bool {
        let rh: u128 = (self.blocks().len() - 1).try_into().unwrap();
        let lh = self
            .blocks
            .values()
            .last()
            .unwrap()
            .blockheight()
            .to_owned();
        rh == lh
    }

    /// This assoc. fxn is on the `Blockchain` struct (rather than `Block`) to
    /// reduce clutter on the `Block` struct. This fxn will be called relatively
    /// infrequently and the functionality is relevant enough to the `Blockchain` class.
    pub fn is_genesis_block(block: &Block) -> bool {
        let block_id = Block::calc_id(block);
        let genesis_hash_str: BlockDigest = [
            66, 211, 46, 10, 157, 18, 155, 115, 197, 147, 50, 48, 80, 109, 218, 216, 188, 202, 161,
            235, 68, 142, 200, 58, 11, 124, 141, 194, 243, 156, 238, 238, 225, 223, 134, 105, 232,
            227, 108, 175, 35, 185, 93, 150, 181, 79, 162, 225, 11, 188, 126, 176, 115, 225, 25,
            187, 179, 152, 208, 13, 176, 94, 192, 249,
        ];

        BlockId::from(block_id) == BlockId(genesis_hash_str)
            && block.id() == BlockId(genesis_hash_str)
    }
}
