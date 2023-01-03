// imports
// local
use crate::common::{init_send_recv, init_users};
use posbc::accounts::{account::Account, accounts::Accounts};

fn util_init_accounts() -> [Account; 4] {
    let users = init_users();

    let send = Account::new(users.send.pbkey(), None);
    let recv = Account::new(users.recv.pbkey(), Some(1));
    let test1 = Account::new(users.test1.pbkey(), Some(22));
    let test2 = Account::new(users.test2.pbkey(), Some(999));

    [send, recv, test1, test2]
}

#[test]
fn create_account_pass() {
    let (send, _recv) = init_send_recv();
    let init_balance = Some(10);
    let new_acct = Account::new(send.pbkey(), init_balance);
    assert_eq!(new_acct.id(), &send.pbkey());
    assert_eq!(new_acct.balance(), &init_balance.unwrap());
    assert_ne!(new_acct.balance(), &(init_balance.unwrap() - 1));
}
/// Check if accounts is created
/// Check if account map - `accounts.accounts` - is created
/// Try to get an account that doesn't exist
#[test]
fn create_accounts_pass() {
    let (send, _recv) = init_send_recv();
    // init account
    let acct1 = Account::new(send.pbkey(), None);
    let accounts = Accounts::new();

    assert_eq!(accounts.len(), 0);
    let null_acct = accounts.get_acct(acct1.id());
    assert!(null_acct.is_none());
}

/// Add an account to accounts if it doesn't already exist
/// Do nothing if account already exists in `Accounts`
///
/// Check if the length of the map has increased
/// Check if an account can be added and retrieved from Accounts struct
#[test]
fn add_new_account_to_accounts_pass() {
    // create init account
    let [acct_to_add, _recv, _test1, _test2] = util_init_accounts();

    // create the accounts struct
    let mut accounts = Accounts::new();
    let accounts_len_pre = accounts.len();
    assert_eq!(accounts_len_pre, 0);

    // check if the account is in the accounts map:
    // 1) add account to map
    accounts.add_acct(acct_to_add);
    let accounts_len_post = accounts.len();
    // 2) check if the length is correct
    assert_eq!(accounts_len_post, accounts_len_pre + 1);
    // 3) get account from map
    let acct_to_add = accounts.get_acct(acct_to_add.id());
    assert!(acct_to_add.is_some())
}

// Purposefully bypass the validation and insert account to the accounts map
