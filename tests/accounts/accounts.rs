// imports
// local
use crate::common::init_send_recv;
use posbc::accounts::{account::Account, accounts::Accounts};

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
    let accounts = Accounts::new();

    let acct = accounts.get_acct(send.pbkey());
    assert_eq!(acct.id(), &send.pbkey());
}
