// imports
// local
use posbc::ledger::{
    txn::{Txn, TxnType},
    txn_pool::{Result, TxnPool},
};

mod common;
use crate::common::init_send_recv;

// TODO: create a fail case (if possible)
#[test]
fn create_txn_pool_pass() {
    let txn_pool = TxnPool::new();
    assert!(txn_pool.txns.len() == 0);
}
#[test]
fn add_txn_pass() -> Result<()> {
    // init
    let (send, recv) = init_send_recv();
    let mut txn_pool = TxnPool::new();

    // create txn
    let txn_1 = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);

    // add to pool
    assert!(txn_pool.txns.len() == 0);
    txn_pool.add_txn(txn_1)?;

    assert!(txn_pool.txns.len() == 1);

    Ok(())
}

#[test]
fn add_txn_fail_dup() -> Result<()> {
    // init
    let (send, recv) = init_send_recv();
    let mut txn_pool = TxnPool::new();

    // create txn
    let txn_1 = Txn::new(send.pbkey(), recv.pbkey(), 100, TxnType::Transfer);
    // add to pool
    txn_pool.add_txn(txn_1)?;
    // should fail
    txn_pool.add_txn(txn_1)?;
    assert!(txn_pool.txns.len() == 1);

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
    let hash_to_remove = txn_1.hash();
    txn_pool.remove_txn(&hash_to_remove)?;
    assert!(txn_pool.txns.len() == 0);

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
    assert!(txn_pool.txns.len() == 1);
    let hash = txn_1.hash;

    assert!(txn_pool.does_txn_exist(&hash));

    Ok(())
}
