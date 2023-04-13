// local
use super::{BlockSignature, Block, BlockId, BlockTxnMap};
use crate::ledger::blockchain::BlockMapKey;

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
    /// Can return `None` if not yet signed.
    pub fn signature(&self) -> Option<&BlockSignature> {
        self.signature.as_ref()
    }
}
