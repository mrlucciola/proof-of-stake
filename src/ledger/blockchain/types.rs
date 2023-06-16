// external
use std::collections::BTreeMap;
// local
use crate::ledger::block::{block_id::BlockId, Block};

/// ### Lookup type for the `blocks` map a string
pub type BlockMapKey = BlockId;
pub type BlockMap = BTreeMap<BlockMapKey, Block>;
