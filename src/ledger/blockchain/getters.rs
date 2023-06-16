use crate::{
    accounts::accounts::{AccountMap, Accounts},
    ledger::{
        block::Block,
        blockchain::{BlockMap, BlockMapKey, Blockchain},
    },
};
use std::borrow::BorrowMut;

impl Blockchain {
    /// ### Get property `Blockchain.blocks`.
    pub fn blocks(&self) -> &BlockMap {
        &self.blocks
    }
    /// ### Get property `Blockchain.block`.
    pub fn block(&self, key: &BlockMapKey) -> Option<&Block> {
        self.blocks.get(key)
    }
    /// ### Get last block on `Blockchain.blocks`.
    /// Current behavior is to panic when no block is present.\
    /// A blockchain should never be empty, representing an undefined state.\
    /// A blockchain always needs a genesis block.
    ///
    /// @todo return a result if empty, not panic.
    pub fn last_block(&self) -> &Block {
        self.blocks.values().next_back().unwrap()
    }
    /// ### Get property `Blockchain.accounts`.
    /// This may be removed in the future.
    /// May be more appropriate in the `node` module.
    pub fn accounts(&self) -> &Accounts {
        &self.accounts
    }
    pub fn accounts_mut(&mut self) -> &mut Accounts {
        self.accounts.borrow_mut()
    }
    /// ### Get map of `accounts`.
    pub fn account_map(&self) -> &AccountMap {
        &self.accounts.accounts()
    }
}
