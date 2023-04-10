// imports
use serde::Serialize;
use std::collections::BTreeMap;

// local
use super::account::{Account, AccountMapKey};

pub type AccountMap = BTreeMap<AccountMapKey, Account>;

#[derive(Debug, Serialize)]
pub struct Accounts {
    accounts: AccountMap,
}

impl Accounts {
    /// Constructor. Creates a data structure (BTreeMap) instance which contains all `Account`s for the blockchain.
    pub fn new() -> Self {
        let accounts = AccountMap::new();

        Self { accounts }
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////
    pub fn accounts(&self) -> &AccountMap {
        &self.accounts
    }
    pub fn accounts_mut(&mut self) -> &mut AccountMap {
        &mut self.accounts
    }

    /// Retrieve account from account map.
    ///
    /// Input is pubkey for consistency across multiple methods.
    ///
    /// In null case, this returns a blank account with the pubkey of the inputted id.\
    /// Either add to the account map and take up space that may never get used or just return a blank account.
    pub fn get_acct(&self, acct_map_key: &AccountMapKey) -> Option<&Account> {
        // TODO: check if pubkey is on curve
        self.accounts.get(acct_map_key)
    }
    /// Returns zero if none
    pub fn acct_balance(&self, acct_map_key: &AccountMapKey) -> u128 {
        match self.get_acct(acct_map_key) {
            Some(a) => a.balance(),
            None => 0u128,
        }
    }
    pub fn get_acct_mut(&mut self, acct_id: &AccountMapKey) -> Option<&mut Account> {
        // TODO: check if pubkey is on curve
        let acct = self.accounts.get_mut(acct_id);

        acct
    }

    /// ## Retrieve or create an account.
    ///
    /// Get from the accounts map, or
    /// initialize and add one if it doesn't exist.
    ///
    /// Same logic as `add_acct()`, but with empty account as the only type of account.
    pub fn get_or_init_acct(&mut self, pbkey_bytes: &AccountMapKey) -> &mut Account {
        self.accounts
            .entry(pbkey_bytes.to_owned())
            .or_insert(Account::new(pbkey_bytes, None))
    }

    pub fn len(&self) -> usize {
        self.accounts.len()
    }
    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// ## Add provided account to the accounts map.
    ///
    /// - If account already exists, return the account.
    /// - Else, add new account to map and return.
    ///
    /// ### @todo
    /// - Should this return an error if no accounts are present?
    /// - Add safeguards.
    pub fn add_acct(&mut self, account: Account) -> &mut Account {
        self.accounts.entry(account.id_key()).or_insert(account)
    }
    pub fn update_acct(&mut self) {}

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
