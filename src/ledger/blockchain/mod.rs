mod getters;
mod setters;
pub mod types;
// external
use serde::Serialize;
// local
use crate::{
    accounts::accounts::Accounts,
    ledger::{
        block::{types::BlockDigest, Block, BlockId},
        general::PbKey,
        wallet::Wallet,
    },
};
pub use types::*;

/// ### Data structure, contains list of sequential block
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

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////

    /// Check if the current blockheight is valid
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

    /// This assoc. fxn is on the `Blockchain` struct (rather than `Block`) to
    /// reduce clutter on the `Block` struct. This fxn will be called relatively
    /// infrequently and the functionality is relevant enough to the `Blockchain` class.
    pub fn is_genesis_block(block: &Block) -> bool {
        let block_id = Block::calc_id(block);
        let genesis_hash_str: BlockDigest = [
            66, 211, 46, 10, 157, 18, 155, 115, 197, 147, 50, 48, 80, 109, 218, 216, 188, 202, 161,
            235, 68, 142, 200, 58, 11, 124, 141, 194, 243, 156, 238, 238, 225, 223, 134, 105, 232,
            227, 108, 175, 35, 185, 93, 150, 181, 79, 162, 225, 11, 188, 126, 176, 115, 225, 25,
            187, 179, 152, 208, 13, 176, 94, 192, 249,
        ];

        BlockId::from(block_id) == BlockId(genesis_hash_str)
            && block.id() == BlockId(genesis_hash_str)
    }

    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}

// @todo move to separate error component
// #[derive(Debug, Error)]
// enum BlockchainError {
//     /// @todo move to `Txn`
//     #[error("Total balance before and after transaction do not match.")]
//     TransactionBalanceMismatch,
//     /// @todo move to `Txn`
//     #[error("Account balance does not change by amount determined by txn.")]
//     AccountBalanceChangeMismatch,
// }
