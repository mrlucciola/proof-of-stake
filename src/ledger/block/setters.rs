// local
use super::{Block, BlockId};
use crate::{
    ledger::{txn::Txn, wallet::Wallet},
    utils::signature::BlockSignature,
};

impl Block {
    /// ### Set the signature for the block.
    fn set_signature(&mut self, signature: BlockSignature) {
        self.signature = Some(signature);
    }

    /// ### Add the signature to the block body in place.
    ///
    /// 1) Sign the block hash
    /// 2) Add signature to `Block` body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> BlockSignature {
        let signature = self.calc_signature(wallet);
        self.set_signature(signature.clone());

        signature
    }

    /// ### Calculate and set the id for a `Block`.
    /// Returns id.
    pub fn set_id(&mut self) -> BlockId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }

    /// ### Add a transaction to the block.
    /// Since we are updating the state of the block, we update the block id (hash) here.
    pub fn add_txn(&mut self, new_txn: Txn) {
        self.txns.entry(new_txn.id_key()).or_insert(new_txn);
        // update block hash since the transactions map has been updated
        self.set_id();
        println!("holp: 33");
    }
}
