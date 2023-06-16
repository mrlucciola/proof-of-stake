// local
use super::{block_header::BlockHeader, Block, BlockId, BlockSignature, BlockTxnMap};
use crate::ledger::{blockchain::BlockMapKey, PbKey};

impl Block {
    /// ### Get property `Block.transactions` included in this `Block`.
    pub fn header(&self) -> &BlockHeader {
        &self.header
    }
    /// ### Get property `Block.transactions` in this `Block`.
    pub fn txns(&self) -> &BlockTxnMap {
        self.header.txns()
    }
    pub fn txns_mut(&mut self) -> &mut BlockTxnMap {
        self.header.txns_mut()
    }
    /// ### Get property `Block.blockheight`.
    pub fn blockheight(&self) -> &u128 {
        self.header.blockheight()
    }
    /// ### Get property `Block.leader`.
    pub fn leader(&self) -> &PbKey {
        self.header.leader()
    }
    /// ### Get property `Block.prev_block_id`.
    pub fn prev_block_id(&self) -> &BlockId {
        self.header.prev_block_id()
    }
    /// ### Get property `Block.id`.
    pub fn id(&self) -> BlockId {
        self.id.unwrap()
    }
    /// ### Get property `Block.id` as lookup key type (for `BlockMap`).
    /// @todo change to byte array
    pub fn id_key(&self) -> BlockMapKey {
        self.id()
    }
    /// ### Get property `Block.signature`.
    /// Currently behavior is to panic if not yet signed.
    pub fn signature(&self) -> BlockSignature {
        self.signature.clone().unwrap()
    }
}
