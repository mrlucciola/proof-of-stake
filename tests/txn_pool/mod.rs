// imports
// local
use posbc::ledger::{
    general::Result,
    txn::{Txn, TxnType},
    txn_pool::TxnPool,
};
// test
use crate::common::init_send_recv;

// TODO: create a fail case
#[test]
fn create_txn_pool_pass() {
    let txn_pool = TxnPool::new();

    assert!(txn_pool.txn_ct() == 0);
}

#[test]
pub fn add_txn_pass() -> Result<()> {
    // init
    let (send, recv) = init_send_recv();
    let mut txn_pool = TxnPool::new();

    // create txn
    let txn_1 = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);

    // add to pool
    assert!(txn_pool.txn_ct() == 0);
    txn_pool.add_txn(txn_1)?;

    assert!(txn_pool.txn_ct() == 1);

    Ok(())
}

#[test]
fn add_txn_fail_dup() -> Result<()> {
    // init
    let (send, recv) = init_send_recv();
    let mut txn_pool = TxnPool::new();

    // create txn
    let txn_1 = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);
    let txn_1_copy = txn_1.clone();
    // add to pool
    txn_pool.add_txn(txn_1)?;
    assert_eq!(txn_pool.txn_ct(), 1);

    // should fail
    if let Ok(_) = txn_pool.add_txn(txn_1_copy) {
        panic!("Adding duplicate txn should fail.")
    };

    assert_eq!(txn_pool.txn_ct(), 1);

    Ok(())
}

#[test]
fn remove_txn_pass() -> Result<()> {
    // init
    let (send, recv) = init_send_recv();
    let mut txn_pool = TxnPool::new();

    // create txn
    let txn_1 = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);

    // add to pool
    txn_pool.add_txn(txn_1.clone())?;

    // remove from pool
    txn_pool.remove_txn(&txn_1)?;
    assert!(txn_pool.txn_ct() == 0);

    Ok(())
}

// TODO: create fail case
#[test]
fn does_txn_exist_pass() -> Result<()> {
    // init
    let (send, recv) = init_send_recv();
    let mut txn_pool = TxnPool::new();

    // create txn
    let txn_1 = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);

    // add to pool
    txn_pool.add_txn(txn_1.clone())?;
    assert!(txn_pool.txn_ct() == 1);

    assert!(txn_pool.does_txn_exist(&txn_1));

    Ok(())
}
