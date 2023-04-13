// local
use super::Blockchain;
use crate::ledger::{
    block::Block,
    txn::Txn,
    txn_pool::{TxnMap, TxnPool},
    Result,
};

impl Blockchain {
    /// ### Process a single `transfer` txn.
    ///
    /// Flow:
    /// - (upstream) node has selected several txns to add to a block, this method moves one of them from the txn pool to the new block.
    /// - validate id - check if the id/digest matches what is calculated from txn.calc_id()
    /// - validate signature - check if signature matches what is calculated from txn.calc_signature()
    /// - validate state change - simulate the state updates:
    ///     - query accounts involved
    ///     - make copies of accts and perform ops specified in the txn
    ///     - return bool if successful
    /// - if txn is valid, apply state changes
    /// - add to block, validate the append (remove on error, i.e. if block is at capacity)
    /// - remove from txn pool
    /// - validate both events (above) happened
    pub fn add_txn_to_block(&mut self, txn: &Txn) -> Result<()> {
        // look up `send` account, decrease their balance
        let acct_send = self
            .accounts
            .get_acct_mut(&txn.pbkey_send().into())
            .unwrap();
        acct_send.decrease_balance(&txn)?;

        // look up `recv` account, increase their balance
        let acct_recv = self.accounts.get_or_init_acct(&txn.pbkey_recv().into());
        acct_recv.increase_balance(&txn)?;

        Ok(())
    }
    /// ### Process a set of `transfer` txns.
    ///
    /// Take txns from an arbitrary list of txns (selected and ordered by leader) and execute them one by one,
    /// applying the state changes to the accounts and placing these transactions
    /// in the specified block.
    ///
    /// @todo optimize by changing txns to preallocated array of hashes (ultimately &str-s)
    /// - This would allow us a set a ceiling limit on the # of txns in a given block
    /// @todo remove txn from mem-pool as they are executed
    pub fn add_txn_to_blocks(
        &mut self,
        txns_to_add: &TxnMap,
        block: &mut Block,
        txn_pool: &mut TxnPool,
    ) -> Result<()> {
        for (_k, txn) in txns_to_add.iter() {
            // @todo which one of these is the valid one? pick one.
            // validate and update account states
            self.add_txn_to_block(&txn)?;
            // add to prospective block
            block.add_txn(txn.clone());

            // #64: remove from txn pool
            txn_pool.remove_txn(&txn)?;
        }

        Ok(())
    }
    /// ### Add a prospective block to the blockchain.
    ///
    /// Block must be signed and pass validation.
    /// - validate block id
    /// - validate block signature
    ///
    /// @todo validate previous block's: 1) height; 2) id. Add error responses for each (InvalidBlockHeight & InvalidBlockId, respectively).
    pub fn add_block(&mut self, block: Block) -> Result<&mut Block> {
        // check if block is valid
        let pbkey = block.leader();
        block.is_valid(&pbkey)?;
        // check if block is signed
        // check if entry exists -> if not, then insert
        Ok(self.blocks.entry(block.id_key()).or_insert(block))
    }
}
