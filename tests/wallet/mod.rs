// imports
use ed25519_dalek::{Digest, Signer};
// local
use posbc::ledger::{
    general::{Sha512, KP},
    txn::{constants::*, TxnCtxDigest, TxnDigest, TxnId},
};
// test
use crate::common::{create_transfer_txn_default, init_send_recv};

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
    let mut prehash: Sha512 = Sha512::new();
    prehash.update(TXN_MSG_CTX);
    prehash.update(msg);
    let digest: TxnDigest = prehash.finalize().into();

    let mut digest_buffer: TxnCtxDigest = [0_u8; TXN_DIGEST_LEN + TXN_SIGNATURE_CTX.len()];
    // add context
    digest_buffer[..TXN_SIGNATURE_CTX.len()].copy_from_slice(TXN_SIGNATURE_CTX);
    // add digest
    digest_buffer[TXN_SIGNATURE_CTX.len()..digest.len() + TXN_SIGNATURE_CTX.len()]
        .copy_from_slice(&digest);
    let ctx_digest: TxnCtxDigest = TxnId(digest).to_presigned_digest();

    // load key and sign
    let kp_send: Vec<u8> = [send.kp.secret.to_bytes(), send.kp.public.to_bytes()].concat();
    let kp = KP::from_bytes(&kp_send).unwrap();
    let msg_signature = kp.sign(&ctx_digest);
    let txn_sig_bytes_manual = msg_signature.to_bytes();

    // sign with txn+wallet method
    let txn_sig_bytes_test = txn2.sign(&send.wallet);
    let txn_sig_bytes_wallet = send.wallet.sign_txn(&txn3);

    let sig_answer_str = b"72249CA1BC217322274989EC2EF7A8191472A41DBEEA889AE871291090E053AA6B5ECCFF3E40A51E96B03B7F0F1645870B6DB5FBD42A2C5B4552DE8B7FA5D30A";
    let txn_sig_bytes_answer: [u8; 64] =
        ed25519_dalek::Signature::from_bytes(&hex::decode(sig_answer_str).unwrap())
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
