// imports
use secp256k1::{Message, Secp256k1};
// local
use posbc::{
    ledger::txn::Txn,
    utils::{hash::BlakeHash, signature::TxnSignature},
};
pub mod common;
use common::{create_transfer_txn, init_send_recv};

#[test]
fn create_unsigned_txn_pass() {
    let id_answer = "d324f1341ed803640717d669a5113c9ef3596728a473982fc8b70bb7d78cc00e";
    let txn = create_transfer_txn();
    let id_str = txn.id_str();

    assert!(id_str == id_answer, "{:?}", id_str);
}

#[test]
fn create_signed_txn_pass() {
    let (send, _recv) = init_send_recv();
    let txn = create_transfer_txn();

    // get signature
    let secp = Secp256k1::new();

    let msg = Message::from_slice(txn.id().as_bytes()).unwrap();
    let signature: TxnSignature = secp.sign_ecdsa(&msg, &send.kp.secret_key()).into();
    let answer = b"30440220136c9d8e942c527262695b2c";
    let answer = BlakeHash::from_bytes(*answer);
    let answer_sig = TxnSignature::sign_id(&answer, &send.kp.secret_key());

    // assert!(signature == answer_sig, "{signature:?}");
}
