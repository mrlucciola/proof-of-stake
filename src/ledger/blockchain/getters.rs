// local
use super::{BlockMap, BlockMapKey, Blockchain};
use crate::{
    accounts::accounts::{AccountMap, Accounts},
    ledger::block::Block,
};

impl Blockchain {
    /// ### Get map of `blocks`.
    /// This mapping represents list of all blocks in commit order.
    pub fn blocks(&self) -> &BlockMap {
        &self.blocks
    }
    /// ### Get `block` by key.
    pub fn block(&self, key: &BlockMapKey) -> Option<&Block> {
        self.blocks.get(key)
    }
    /// ### Get most recently committed `block`.
    /// Current behavior is to panic when no block is present.\
    /// A blockchain should never be empty, representing an undefined state.\
    /// A blockchain always needs a genesis block.
    pub fn last_block(&self) -> &Block {
        self.blocks.values().next_back().unwrap()
    }
    /// ### Get map of all `accounts`.
    /// This may be removed in the future.
    /// May be more appropriate in the `node` module.
    pub fn accounts(&self) -> &Accounts {
        &self.accounts
    }
    /// ### Get map of `accounts`.
    pub fn account_map(&self) -> &AccountMap {
        &self.accounts.accounts()
    }
}
