// local
use super::{Block, BlockSignature};
use crate::ledger::{txn::Txn, wallet::Wallet};

impl Block {
    /// ## Add the signature to the block body in place.
    ///
    /// 1) Sign the block hash
    /// 2) Add signature to `Block` body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> BlockSignature {
        let signature = self.calc_signature(wallet);
        self.set_signature(signature.clone());

        // @todo hash and sign again
        self.update_id();
        let signature = self.calc_signature(wallet);
        self.set_signature(signature.clone());

        signature
    }

    /// ## Add a transaction to the block.
    /// Since we are updating the state of the block, we update the block id (hash) here.
    pub fn add_txn(&mut self, new_txn: Txn) {
        self.txns_mut()
            .entry(new_txn.id_key().to_owned())
            .or_insert(new_txn);
        // update block hash since the transactions map has been updated
        self.update_id();
    }
}
