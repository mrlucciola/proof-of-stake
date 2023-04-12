// imports
use ed25519_dalek::{Digest, Sha512};
// local
use posbc::{
    ledger::txn::TXN_MSG_CTX,
    utils::signature::{TxnSignature, TXN_SIGNATURE_CONTEXT},
};
// test
use crate::common::{create_transfer_txn_default, init_send_recv};

#[test]
fn create_unsigned_txn_pass() {
    let id_answer: [u8; 64] = [
        21, 113, 200, 54, 53, 32, 176, 56, 194, 13, 161, 151, 94, 184, 187, 170, 96, 237, 102, 8,
        175, 16, 204, 109, 189, 112, 94, 171, 41, 194, 31, 155, 83, 195, 110, 130, 195, 63, 119,
        239, 198, 164, 165, 175, 60, 26, 128, 114, 35, 117, 143, 156, 254, 180, 151, 93, 80, 69,
        81, 72, 146, 143, 215, 41,
    ];
    let txn = create_transfer_txn_default();
    let id_test = txn.calc_id();

    assert!(
        id_answer == id_test,
        "\nanswer: {:?}\n  test: {:?}",
        id_answer,
        id_test,
    );
}

#[test]
fn create_signed_txn_pass() {
    let (send, _recv) = init_send_recv();
    let kp_send: Vec<u8> = [send.kp.secret.to_bytes(), send.kp.public.to_bytes()].concat();
    let kp = ed25519_dalek::Keypair::from_bytes(&kp_send).unwrap();

    // calc signature manually
    let txn = create_transfer_txn_default();
    let mut txn1 = txn.clone();
    let txn2 = txn.clone();
    let mut txn3 = txn.clone();
    let txn4 = txn.clone();

    let msg = txn1.to_bytes();
    // Create a hash digest object which we'll feed the message into:
    let mut prehashed_1: Sha512 = Sha512::new();
    // add the txn context
    prehashed_1.update(TXN_MSG_CTX);
    // add the txn bytes
    prehashed_1.update(msg.clone());

    let msg_signature_manual = kp
        .sign_prehashed(prehashed_1, Some(TXN_SIGNATURE_CONTEXT))
        .unwrap();

    // calc signature using methods
    let msg_sig_txn_sign: TxnSignature = txn1.sign(&send.wallet);
    let msg_sig_txn_calc: TxnSignature = txn2.calc_signature(&send.wallet);
    txn3.set_signature(txn3.calc_signature(&send.wallet));
    let msg_sig_txn_set: TxnSignature = txn3.signature().clone();
    let msg_signature_wallet: TxnSignature = send.wallet.sign_txn(&txn4);

    assert_eq!(
        msg_signature_manual,
        msg_sig_txn_sign.clone().into(),
        "0: \n{msg_signature_manual:?}\n{msg_sig_txn_sign:?}"
    );
    assert_eq!(
        msg_sig_txn_sign, msg_sig_txn_calc,
        "1: \n{msg_sig_txn_sign:?}\n{msg_sig_txn_calc:?}"
    );
    assert_eq!(
        msg_sig_txn_calc, msg_sig_txn_set,
        "2: \n{msg_sig_txn_calc:?}\n{msg_sig_txn_set:?}"
    );
    assert_eq!(
        msg_sig_txn_set, msg_signature_wallet,
        "3: \n{msg_sig_txn_set:?}\n{msg_signature_wallet:?}"
    );
}
