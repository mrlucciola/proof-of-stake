// imports
use std::collections::BTreeMap;
// local
use super::account::{Account, AccountId, AccountMapkey};
use crate::ledger::general::Result;

type AccountMap = BTreeMap<AccountMapkey, Account>;

pub struct Accounts {
    accounts: AccountMap, // BTreeMap<AcctKey, Acct>
}

impl Accounts {
    /// Constructor
    pub fn new() {}
    /// TODO: add safeguards
    pub fn add_acct(&mut self, account: Account) {
        self.accounts.entry(account.id_key()).or_insert(account);
    }
    pub fn update_acct(&mut self) {}
    /// How to handle empty accounts?
    ///
    /// Either add to the account map and take up space that may never get used or just return a blank account.
    /// When attempting to retrieve the account, there should be proper error handling
    pub fn get_acct(&self, acct_id: AccountId) -> Result<Account> {
        // TODO: check if pubkey is on curve
        let acct = self.accounts.get(&acct_id.to_string());

        match acct {
            Some(a) => Ok(*a),
            // Create a new account, clone lasts as long as acct_id
            None => Ok(Account::new(acct_id, None)),
        }
    }
}
