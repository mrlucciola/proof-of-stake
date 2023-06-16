use crate::ledger::{block::Block, blockchain::Blockchain};

impl Blockchain {
    /// ### Check if the current blockheight is valid
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

    /// ### Check if a given block is the genesis block on current instance of blockchain.
    pub fn is_genesis_block(&self, block: &Block) -> bool {
        let genesis = self.blocks().values().next().unwrap();

        block.id() == genesis.calc_id() && block.calc_id() == genesis.id()
    }
}
