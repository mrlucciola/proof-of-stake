// imports
use secp256k1::PublicKey;
// local

/// Representation of a single on-chain account.
///
/// TODO: add `rent`
#[derive(Debug, Clone, Copy)]
pub struct Account {
    /// ID: An on-chain account's public identifier, as its represented throughout the rest of the repositories. Abstract class, derives public key
    id: AccountId,
    /// Amount of blockchain token stored in account. Can be transfered to other accounts or contracts
    balance: u128,
}

pub type AccountId = PublicKey;
pub type AccountPbkey = PublicKey;
pub type AccountMapkey = String;

impl Account {
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
    pub fn id(&self) -> &AccountId {
        &self.id
    }
    pub fn id_pbkey(&self) -> &AccountPbkey {
        self.id()
    }
    pub fn id_str(&self) -> String {
        self.id().to_string()
    }
    /// Get the lookup key for Account ID in hash maps throughout the application.
    pub fn id_key(&self) -> AccountMapkey {
        self.id_str()
    }
    pub fn balance(&self) -> &u128 {
        &self.balance
    }
    ////////////////////////// GETTERS //////////////////////////
    /////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////
    ////////////////////////// SETTERS //////////////////////////
    pub fn set_id(&mut self, new_acct_id: AccountId) {
        self.id = new_acct_id;
    }
    ////////////////////////// SETTERS //////////////////////////
    /////////////////////////////////////////////////////////////
}
