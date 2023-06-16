use crate::ledger::block::{block_id::BlockId, Block};
use std::collections::BTreeMap;

/// ### Lookup type for the `blocks` map a string
pub type BlockMapKey = BlockId;
pub type BlockMap = BTreeMap<BlockMapKey, Block>;
