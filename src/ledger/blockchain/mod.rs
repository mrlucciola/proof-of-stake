mod error;
mod getters;
mod setters;
pub mod types;
mod validation;
// external
use serde::Serialize;
// local
use crate::{accounts::accounts::Accounts, ledger::blockchain::types::*};

/// ## Data structure, contains list of sequential blocks.
#[derive(Debug, Serialize)]
pub struct Blockchain {
    /// Ordered lookup collection (BTreeMap) of blocks, queriable by their ID.
    blocks: BlockMap,
    /// Ordered lookup collection (BTreeMap) of accounts, queriable by their ID, wrapped with methods.
    accounts: Accounts,
}
impl Blockchain {
    /// ### Initialize a new `Blockchain` instance.
    /// Contains an ordered mapping of blocks, and an ordered mapping of accounts.
    pub fn new() -> Self {
        Self {
            blocks: BlockMap::new(),
            accounts: Accounts::new(),
        }
    }
}
