// imports
// local
use posbc::ledger::{
    general::Result,
    txn::{Txn, TxnType},
    txn_pool::TxnPool,
};

pub mod common;
use common::init_send_recv;

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
    let txn_1_copy = Txn { ..txn_1 };
    // add to pool
    txn_pool.add_txn(txn_1)?;
    // should fail
    txn_pool.add_txn(txn_1_copy)?;
    assert!(txn_pool.txn_ct() == 1);

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
    let id_to_remove = txn_1.id();
    txn_pool.remove_txn(&id_to_remove)?;
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
    let id = txn_1.id;

    assert!(txn_pool.does_txn_exist(&id));

    Ok(())
}
