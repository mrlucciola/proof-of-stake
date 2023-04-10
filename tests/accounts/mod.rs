// imports
// local
use posbc::{
    accounts::{account::Account, accounts::Accounts},
    ledger::general::Result,
};
// test
use crate::common::{create_transfer_txn, init_send_recv, init_users};

// util
fn util_init_accounts() -> [Account; 4] {
    let users = init_users();

    let send = Account::new(users.send.pbkey().as_bytes(), None);
    let recv = Account::new(users.recv.pbkey().as_bytes(), Some(1));
    let test1 = Account::new(users.test1.pbkey().as_bytes(), Some(22));
    let test2 = Account::new(users.test2.pbkey().as_bytes(), Some(999));

    [send, recv, test1, test2]
}

#[test]
fn create_account_pass() {
    let (send, _recv) = init_send_recv();
    let init_balance = Some(10);
    let new_acct = Account::new(send.pbkey().as_bytes(), init_balance);
    assert_eq!(new_acct.id(), send.pbkey().as_bytes());
    assert_eq!(new_acct.balance(), init_balance.unwrap());
    assert_ne!(new_acct.balance(), (init_balance.unwrap() - 1));
}
/// Check if accounts is created
/// Check if account map - `accounts.accounts` - is created
/// Try to get an account that doesn't exist
#[test]
fn create_accounts_pass() {
    let (send, _recv) = init_send_recv();
    // init account
    let acct1 = Account::new(send.pbkey().as_bytes(), None);
    let accounts = Accounts::new();

    assert_eq!(accounts.len(), 0);
    let null_acct = accounts.get_acct(&acct1.id_key());
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
    let acct_to_add = accounts.get_acct(&acct_to_add.id_key());
    assert!(acct_to_add.is_some())
}

// Purposefully bypass the validation and insert account to the accounts map

#[test]
fn balance_incr_pass() -> Result<()> {
    let [mut send, _recv, _test1, _test2] = util_init_accounts();
    let balance_pre = send.balance();
    let amt_to_incr = 1;
    let txn = create_transfer_txn(amt_to_incr);

    send.increase_balance(&txn)?;

    let balance_post = send.balance();

    assert_eq!(balance_post, 1, "Balance should have increased to 1");
    assert_eq!(
        balance_post - balance_pre,
        amt_to_incr,
        "Balance should have increased by 1"
    );

    Ok(())
}

/// decrease balance below 0
#[test]
#[should_panic]
fn balance_decr_fail() {
    let [_send, mut recv, _test1, _test2] = util_init_accounts();
    let amt_to_decr = 2;
    let txn = create_transfer_txn(amt_to_decr);

    // should fail because data type is unsigned int and we are attempting to make it negative
    recv.decrease_balance(&txn).unwrap();
}
#[test]
fn balance_decr_pass() -> Result<()> {
    let [_send, mut recv, _test1, _test2] = util_init_accounts();
    let balance_pre = recv.balance();
    let amt_to_decr = 1;
    let txn = create_transfer_txn(amt_to_decr);

    // decrease balance to 0
    recv.decrease_balance(&txn)?;

    let balance_post = recv.balance();

    assert_eq!(balance_post, 0, "Balance should have decreased to 0");
    assert_eq!(
        balance_pre - balance_post,
        amt_to_decr,
        "Balance should have decreased by 1"
    );

    Ok(())
}
