// imports
use {anyhow::ensure, secp256k1::PublicKey, serde::Serialize};
// local
use crate::ledger::{
    general::{PbKey, Result},
    txn::{Txn, TxnType},
};

/// Representation of a single on-chain account.
///
/// TODO: add `rent`
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Account {
    /// ID: An on-chain account's public identifier, as its represented throughout the rest of the repositories. Abstract class, derives public key
    id: AccountId,
    /// Amount of blockchain token stored in account. Can be transfered to other accounts or contracts
    balance: u128,
}

pub type AccountId = [u8; 32];
pub type AccountPbkey = PublicKey;
pub type AccountMapKey = AccountId;

impl Account {
    /// Constructor: Create instance of an on-chain account.
    ///
    /// Load account info from the blockchain.
    pub fn new(id: &AccountId, balance: Option<u128>) -> Self {
        // set default balance value
        let balance = match balance {
            Some(b) => b,
            None => 0u128,
        };

        Self {
            id: id.clone(),
            balance,
        }
    }
    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////
    pub fn id(&self) -> &AccountId {
        &self.id
    }
    pub fn id_pbkey(&self) -> PbKey {
        PbKey(self.id().to_owned())
    }
    pub fn id_str(&self) -> String {
        String::from_utf8(self.id().to_vec()).unwrap()
    }
    /// Get the lookup key for Account ID in hash maps throughout the application.
    pub fn id_key(&self) -> AccountMapKey {
        self.id().to_owned()
    }
    pub fn balance(&self) -> u128 {
        self.balance
    }
    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////
    // fn set_id(&mut self, new_acct_id: AccountId) {
    //     self.id = new_acct_id;
    // }

    /// ## Increase balance by set amount.
    ///
    /// New balance must be greater than original balance.
    ///
    /// Balance must be above 0.
    ///
    /// Should only execute if transfer txn or coinbase txn.
    pub fn increase_balance(&mut self, txn: &Txn) -> Result<u128> {
        // @todo add coinbase validation + error
        if txn.txn_type != TxnType::Transfer {
            panic!("Only allowing transfer transactions.");
        }

        let amt_to_incr = txn.amt;
        self.balance += amt_to_incr;

        Ok(self.balance)
    }
    /// ## Decrease balance by set amount.
    ///
    /// New balance must be less than original balance.
    ///
    /// Balance must be above 0.
    ///
    /// Should only execute if transfer txn or fee txn.
    pub fn decrease_balance(&mut self, txn: &Txn) -> Result<u128> {
        // @todo add fee validation + error
        ensure!(
            txn.txn_type == TxnType::Transfer,
            "Transfer is the only accepted Txn Type"
        );

        let amt_to_decr = txn.amt;
        self.balance -= amt_to_decr;

        Ok(self.balance)
    }
    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}
