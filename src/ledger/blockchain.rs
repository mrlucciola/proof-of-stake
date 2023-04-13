// external
use serde::Serialize;
use std::collections::BTreeMap;
// local
use super::{
    block::{types::BlockDigest, Block, BlockId},
    general::{PbKey, PubKey, Result},
    txn::Txn,
    txn_pool::{TxnMap, TxnPool},
    wallet::Wallet,
};
use crate::accounts::accounts::{AccountMap, Accounts};

/// ### Lookup type for the `blocks` map a string
pub type BlockMapKey = BlockId;
pub type BlockMap = BTreeMap<BlockId, Block>;

/// ### Data structure, contains list of sequential block
#[derive(Debug, Serialize)]
pub struct Blockchain {
    /// Ordered lookup collection of blocks.
    blocks: BlockMap,
    /// Ordered lookup collection of accounts, wrapped with methods.
    pub accounts: Accounts,
    /// Pubkey of the entity used to initialize the blockchain.
    pub initializer: PubKey,
}
impl Blockchain {
    /// ## Create new `Blockchain` instance.
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
    ////////////////////////////// GETTERS //////////////////////////////

    /// ### Getter for `blocks` map
    pub fn blocks(&self) -> &BlockMap {
        &self.blocks
    }
    /// ### Get block by key
    pub fn block(&self, key: &BlockMapKey) -> Option<&Block> {
        self.blocks.get(key)
    }
    /// ### Get most recently committed block
    /// We unwrap because blockchain should never be empty, representing an undefined state.
    pub fn last_block(&self) -> &Block {
        self.blocks.values().next_back().unwrap()
    }
    /// ### Getter for `accounts`
    pub fn accounts(&self) -> &Accounts {
        &self.accounts
    }
    pub fn account_map(&self) -> &AccountMap {
        &self.accounts.accounts()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// ## Add a prospective block to the blockchain.
    ///
    /// Block must be signed and pass validation.
    ///
    /// @todo validate previous block's: 1) height; 2) id. Add error responses for each (InvalidBlockHeight & InvalidBlockId, respectively).
    pub fn add_block(&mut self, block: Block) -> Result<&mut Block> {
        // check if block is valid
        let pbkey = PbKey::from_bytes(&block.leader)?;
        block.is_valid(&pbkey)?;
        // check if block is signed
        // check if entry exists -> if not, then insert
        Ok(self.blocks.entry(block.id_key()).or_insert(block))
    }

    /// @todo Validate txn, then process
    /// ## Context:
    ///
    /// - Leader has grouped several txns into a prospective block
    /// - They execute each txn in serial, updating the accounts in order
    pub fn process_transfer_txn(&mut self, txn: &Txn) -> Result<()> {
        // look up `send` account, decrease their balance
        let acct_send = self
            .accounts
            .get_acct_mut(txn.pbkey_send().as_bytes())
            .unwrap();
        acct_send.decrease_balance(&txn)?;

        // look up `recv` account, increase their balance
        let acct_recv = self.accounts.get_or_init_acct(txn.pbkey_recv().as_bytes());
        acct_recv.increase_balance(&txn)?;

        Ok(())
    }
    /// ## Process a set of `transfer` txns.
    ///
    /// Take txns from an arbitrary list of txns and execute them one by one,
    /// applying the state changes to the accounts and placing these transactions
    /// in the specified block.
    ///
    /// @todo optimize by changing txns to preallocated array of hashes (ultimately &str-s)
    /// - This would allow us a set a ceiling limit on the # of txns in a given block
    /// @todo remove txn from mem-pool as they are executed
    pub fn process_transfer_txns(
        &mut self,
        txns_to_add: &TxnMap,
        block: &mut Block,
        txn_pool: &mut TxnPool,
    ) -> Result<()> {
        for (_k, txn) in txns_to_add.iter() {
            // #64: remove from txn pool
            let txn = txn_pool.remove_txn(&txn)?;
            // validate and update account states
            self.process_transfer_txn(&txn)?;

            // add to prospective block
            block.add_txn(txn);
        }

        Ok(())
    }

    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////

    /// Check if the current blockheight is valid
    /// @todo call this after block genesis creation
    pub fn is_blockheight_valid(&self) -> bool {
        self.blocks.values().last().unwrap().blockheight()
            == (self.blocks().len() - 1).try_into().unwrap()
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
