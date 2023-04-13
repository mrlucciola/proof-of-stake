mod error;
mod getters;
mod setters;
pub mod types;
mod validation;
// external
use serde::Serialize;
// local
use crate::{
    accounts::accounts::Accounts,
    ledger::{block::Block, general::PbKey, wallet::Wallet},
};
pub use types::*;

/// ### Data structure, contains list of sequential blocks.
#[derive(Debug, Serialize)]
pub struct Blockchain {
    /// Ordered lookup collection of blocks.
    blocks: BlockMap,
    /// Ordered lookup collection of accounts, wrapped with methods.
    /// This may be removed in the future.
    /// May be more appropriate in the `node` module.
    pub accounts: Accounts,
    /// Pubkey of the entity used to initialize the blockchain.
    pub initializer: PbKey,
}
impl Blockchain {
    /// ### Create new `Blockchain` instance.
    ///
    /// Contains list (BTreeMap) of blocks in sequence, queriable by their ID, in string form.
    ///
    /// The first block in a blockchain is the genesis block.
    pub fn new() -> Self {
        let blocks = BlockMap::new();
        let accounts = Accounts::new();
        // @todo - this is not acceptable for production
        let initializer_wallet =
            Wallet::new_from_file(&"hidden/master_key_ed25519.json".to_string());
        let mut blockchain = Self {
            blocks,
            accounts,
            initializer: initializer_wallet.pbkey().into(),
        };

        // Create the genesis block - panic if unexpected behavior
        let mut genesis_block = Block::new_genesis(initializer_wallet.pbkey()).unwrap();
        genesis_block.sign(&initializer_wallet);
        // @todo handle error properly
        blockchain.add_block(genesis_block).unwrap();

        blockchain
    }
}
