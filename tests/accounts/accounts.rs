use posbc::{
    accounts::{account::Account, accounts::Accounts},
    ledger::{txn::Txn, wallet::Wallet},
    utils::signature::TxnSignature,
};
// pub mod common;
use crate::common::{create_transfer_txn, init_send_recv};

#[test]
fn create_account_pass() {
    let (send, _recv) = init_send_recv();
    let init_balance = Some(10);
    let new_acct = Account::new(send.pbkey(), init_balance);
    assert_eq!(new_acct.id(), &send.pbkey());
    assert_eq!(new_acct.balance(), &init_balance.unwrap());
    assert_ne!(new_acct.balance(), &(init_balance.unwrap() - 1));
}
fn create_accounts_pass() {
    let accounts = Accounts::new();
}
