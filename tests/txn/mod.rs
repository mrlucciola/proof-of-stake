// imports
use ed25519_dalek::{Digest, Signer};
// local
use posbc::ledger::{
    general::{Sha512, KP},
    txn::{constants::*, TxnCtxDigest, TxnDigest, TxnId, TxnSignature},
};
// test
use crate::common::{create_transfer_txn_default, init_send_recv};

#[test]
fn create_unsigned_txn_pass() {
    let id_answer: TxnDigest = [
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
    let kp = KP::from_bytes(&kp_send).unwrap();

    let txn = create_transfer_txn_default();
    let mut txn1 = txn.clone();
    let txn2 = txn.clone();
    let txn3 = txn.clone();

    // calc signature manually
    let mut prehash: Sha512 = Sha512::new();
    // add the txn version
    prehash.update(TXN_MSG_CTX);
    // add the txn bytes
    prehash.update(txn1.to_bytes());
    // convert to byte array
    let digest: TxnDigest = prehash.finalize().into();
    let mut digest_buffer: TxnCtxDigest = [0_u8; TXN_DIGEST_LEN + TXN_SIGNATURE_CTX.len()];
    // add context
    digest_buffer[..TXN_SIGNATURE_CTX.len()].copy_from_slice(TXN_SIGNATURE_CTX);
    // add digest
    digest_buffer[TXN_SIGNATURE_CTX.len()..digest.len() + TXN_SIGNATURE_CTX.len()]
        .copy_from_slice(&digest);
    let ctx_digest: TxnCtxDigest = TxnId(digest).to_presigned_digest();

    // sign msg and return signature
    let msg_signature_manual = kp.sign(&ctx_digest);

    // calc signature using methods
    let msg_sig_txn_sign: TxnSignature = txn1.sign(&send.wallet);
    let msg_sig_txn_calc: TxnSignature = txn2.calc_signature(&send.wallet);
    let msg_signature_wallet: TxnSignature = send.wallet.sign_txn(&txn3);

    assert_eq!(
        msg_signature_manual,
        msg_sig_txn_sign.clone().into(),
        "\n0: \n{msg_signature_manual:?}\n{}",
        msg_sig_txn_sign.to_str()
    );
    assert_eq!(
        msg_sig_txn_sign,
        msg_sig_txn_calc,
        "\n1: \n{msg_sig_txn_sign:?}\n{}",
        msg_sig_txn_calc.to_str()
    );
    assert_eq!(
        msg_sig_txn_calc,
        msg_signature_wallet,
        "\n2: \n{msg_sig_txn_calc:?}\n{}",
        msg_signature_wallet.to_str()
    );
}
