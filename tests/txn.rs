// imports
use arrayvec::ArrayString;
use blake3::OUT_LEN;
use secp256k1::{Message, Secp256k1};
// local
use posbc::utils::{hash::BlakeHash, signature::TxnSignature};
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
    let signature: TxnSignature = secp.sign_ecdsa(&msg, &send.kp.secret_key());

    let mut answer_arr_str = ArrayString::<{ 2 * OUT_LEN }>::new();
    answer_arr_str.push_str("d324f1341ed803640717d669a5113c9ef3596728a473982fc8b70bb7d78cc00e");

    let answer = BlakeHash::from(answer_arr_str);
    let msg_a = Message::from_slice(answer.as_bytes()).unwrap();
    let answer_sig = send.wallet.sign_txn(&answer);
    let txn_sig: TxnSignature = txn.calc_signature(&send.wallet);

    assert_eq!(signature, answer_sig, "1: {msg_a:?}");
    assert_eq!(signature, txn_sig, "2: {signature:?}");
}
