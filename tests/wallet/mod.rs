// imports
use ed25519_dalek::{Digest, Sha512};
// local
// test
use crate::common::{create_transfer_txn_default, init_send_recv};
use posbc::{ledger::txn::TXN_MSG_CTX, utils::signature::TXN_SIGNATURE_CONTEXT};

#[test]
fn verify_txn_signature_pass() {
    // init
    let (send, _recv) = init_send_recv();
    let txn = create_transfer_txn_default();
    let txn1 = txn.clone();
    let mut txn2 = txn.clone();
    let txn3 = txn.clone();

    // manually calculate
    let msg = txn1.to_bytes();
    let mut prehashed: Sha512 = Sha512::new();
    prehashed.update(TXN_MSG_CTX);
    prehashed.update(msg);

    // load key and sign
    let kp_send: Vec<u8> = [send.kp.secret.to_bytes(), send.kp.public.to_bytes()].concat();
    let kp = ed25519_dalek::Keypair::from_bytes(&kp_send).unwrap();
    let msg_signature = kp
        .sign_prehashed(prehashed, Some(TXN_SIGNATURE_CONTEXT))
        .unwrap();
    let txn_sig_bytes_manual = msg_signature.to_bytes();

    // sign with txn+wallet method
    let txn_sig_bytes_test = txn2.sign(&send.wallet);
    let txn_sig_bytes_wallet = send.wallet.sign_txn(&txn3);

    let answer_bytes_raw: [u8; 64] = [
        217, 116, 3, 122, 20, 126, 184, 9, 1, 248, 248, 216, 26, 26, 23, 56, 69, 95, 95, 140, 58,
        216, 237, 55, 57, 22, 149, 113, 84, 127, 108, 218, 84, 249, 138, 121, 151, 171, 168, 194,
        102, 109, 175, 163, 8, 23, 219, 101, 235, 105, 12, 80, 190, 84, 3, 87, 86, 255, 218, 115,
        25, 217, 224, 10,
    ];
    let txn_sig_bytes_answer: [u8; 64] = ed25519_dalek::Signature::from_bytes(&answer_bytes_raw)
        .unwrap()
        .to_bytes();

    assert_eq!(
        &txn_sig_bytes_test,
        &txn_sig_bytes_answer.into(),
        "  test-answer: \n  test: {txn_sig_bytes_test:?}\nanswer: {txn_sig_bytes_answer:?}"
    );
    assert_eq!(
        &txn_sig_bytes_manual, &txn_sig_bytes_answer,
        "manual-answer: \nmanual: {txn_sig_bytes_manual:?}\nanswer: {txn_sig_bytes_answer:?}"
    );
    assert_eq!(
        &txn_sig_bytes_wallet,
        &txn_sig_bytes_answer.into(),
        "wallet-answer: \nwallet: {txn_sig_bytes_wallet:?}\nanswer: {txn_sig_bytes_answer:?}"
    );
}
