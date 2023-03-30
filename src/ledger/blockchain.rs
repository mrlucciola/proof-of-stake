// imports
use serde::Serialize;
use std::collections::BTreeMap;
// local
use super::{
    blocks::{Block, BlockId, BlockTxnMap},
    general::{PbKey, Result},
    txn::Txn,
    txn_pool::{TxnMap, TxnPool},
    wallet::Wallet,
};
use crate::accounts::accounts::{AccountMap, Accounts};

/// Lookup type for the `blocks` map a string
pub type BlockMapKey = String; // TODO: change to hex
pub type BlockMap = BTreeMap<BlockMapKey, Block>;

/// Data structure, contains list of sequential block
#[derive(Debug, Serialize)]
pub struct Blockchain {
    /// Ordered lookup collection of blocks.
    blocks: BlockMap,
    /// Ordered lookup collection of accounts, wrapped with methods.
    pub accounts: Accounts,
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

        let mut blockchain = Self { blocks, accounts };

        // Create the genesis block - panic if unexpected behavior
        blockchain.genesis().unwrap();

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
    ///
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
        block.is_valid(&block.leader)?;
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
        let map_key_send = &txn.pbkey_send.to_string();
        let acct_send = self.accounts.get_acct_mut(&map_key_send).unwrap();
        acct_send.decrease_balance(&txn)?;

        // look up `recv` account, increase their balance
        let acct_recv = self.accounts.get_or_init_acct(&txn.pbkey_recv);
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

    /// ## Create and add the genesis block.
    ///
    /// The genesis block is the initial/seed block for the entire blockchain.
    ///
    /// Notes:
    /// - Validates that no prior blocks exist.
    /// - Manually assigns a blocktime (0 = 1/1/1970 00:00).
    fn genesis(&mut self) -> Result<()> {
        if !self.blocks().is_empty() {
            panic!("Blockchain needs to be empty")
        }

        let leader_wallet = Wallet::new_from_file(&"hidden/master_key.json".to_string());
        let leader: PbKey = leader_wallet.pbkey();

        // create a new block using the `Block` constructor - we need to replace the blockheight, id, and signature
        let mut genesis_block = Block::new(
            BlockTxnMap::new(),
            leader,
            BlockId::from_bytes([0u8; 32]),
            0,
        );
        // replace blockheight & time
        genesis_block.blockheight = 0;
        genesis_block.system_time = 0;
        // replace id/hash
        genesis_block.set_id();
        genesis_block.sign(&leader_wallet);

        self.add_block(genesis_block)?;

        Ok(())
    }

    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////

    /// Check if the current blockheight is valid
    pub fn is_blockheight_valid(&self) -> bool {
        self.blocks.values().last().unwrap().blockheight
            == (self.blocks().len() - 1).try_into().unwrap()
    }

    /// This assoc. fxn is on the `Blockchain` struct (rather than `Block`) to
    /// reduce clutter on the `Block` struct. This fxn will be called relatively
    /// infrequently and the functionality is relevant enough to the `Blockchain` class.
    pub fn is_genesis_block(block: &Block) -> bool {
        let block_id = Block::calc_id(block);
        let genesis_hash_str = "444cb81543bd99399e390d2565091b87f38149436e2d6db6d2df3bf5cc970e2c";

        block_id.to_string() == genesis_hash_str && block.id().to_string() == genesis_hash_str
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
