use ed25519_dalek::{Digest, Signer};
use posbc::ledger::{
    general::{HashAlgo, KP},
    txn::{constants::*, txn_id::TxnId, types::*},
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
    let msg = txn1.header().serialize();
    let mut prehash = HashAlgo::new();
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

    assert_eq!(
        &txn_sig_bytes_test,
        &txn_sig_bytes_manual.into(),
        "manual-answer: \nmanual: {txn_sig_bytes_test:?}\nanswer: {txn_sig_bytes_manual:?}"
    );
    assert_eq!(
        &txn_sig_bytes_test,
        &txn_sig_bytes_wallet.clone().into(),
        "wallet-answer: \nwallet: {txn_sig_bytes_wallet:?}\nanswer: {txn_sig_bytes_test:?}"
    );
}
