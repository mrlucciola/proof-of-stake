// local
use super::{Block, BlockId, BlockSignature, BlockTxnMap};
use crate::ledger::{blockchain::BlockMapKey, PbKey};

impl Block {
    /// ### Get `Block.id` property.
    pub fn id(&self) -> BlockId {
        self.id.unwrap()
    }
    /// ### Get `Block.id` in type used as a key for `BlockMap`.
    /// @todo change to byte array
    pub fn id_key(&self) -> BlockMapKey {
        self.id()
    }
    /// ### Get `Block.transactions` included in this `Block`.
    pub fn txns(&self) -> &BlockTxnMap {
        &self.txns
    }
    /// ### Get `Block.signature` property.
    /// Currently behavior is to panic if not yet signed.
    pub fn signature(&self) -> BlockSignature {
        self.signature.clone().unwrap()
    }
    /// ### Get `Block.blockheight` property.
    pub fn blockheight(&self) -> &u128 {
        &self.blockheight
    }
    /// ### Get `Block.leader` property.
    pub fn leader(&self) -> &PbKey {
        &self.leader
    }
}
