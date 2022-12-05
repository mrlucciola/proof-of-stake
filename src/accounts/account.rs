// imports
use secp256k1::PublicKey;
// local

/// Representation of a single on-chain account.
///
pub struct Acct {
    /// ID: An on-chain account's public identifier, as its represented throughout the rest of the repositories. Abstract class, derives public key
    id: AcctId,
    /// Amount of blockchain token stored in account. Can be transfered to other accounts or contracts
    balance: u128,
}

pub type AcctId = PublicKey;
pub type AcctPbkey = PublicKey;
pub type AcctMapkey = String;

impl Acct {
    /// Constructor: Create instance of an on-chain account.
    ///
    /// This assoc. fxn does NOT create an account on chain.
    ///
    /// Load account info from the blockchain.
    pub fn new(id: PublicKey, balance: Option<u128>) -> Self {
        // set default balance value
        let balance = match balance {
            Some(b) => b,
            None => 0u128,
        };

        Self { id, balance }
    }
    /////////////////////////////////////////////////////////////
    ////////////////////////// GETTERS //////////////////////////
    pub fn id(&self) -> &AcctId {
        &self.id
    }
    pub fn id_pbkey(&self) -> &AcctPbkey {
        self.id()
    }
    pub fn id_str(&self) -> String {
        self.id().to_string()
    }
    /// Get the lookup key for Account ID in hash maps throughout the application.
    pub fn id_key(&self) -> AcctMapkey {
        self.id_str()
    }
    pub fn balance(&self) -> &u128 {
        &self.balance
    }
    ////////////////////////// GETTERS //////////////////////////
    /////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////
    ////////////////////////// SETTERS //////////////////////////
    pub fn set_id(&mut self, new_acct_id: AcctId) {
        self.id = new_acct_id;
    }
    ////////////////////////// SETTERS //////////////////////////
    /////////////////////////////////////////////////////////////
}
