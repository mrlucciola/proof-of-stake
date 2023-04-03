// imports
use secp256k1::{Message, Secp256k1};
use std::str::FromStr;
// local
// test
use crate::common::{create_transfer_txn_default, init_send_recv};

#[test]
fn verify_txn_signature_pass() {
    // init
    let answer_str = "304402203277d88a0cc247efb4677b7e8617f5fbb71d9150d13b45fc578664777b611740022044f62b8748cb3a5e6d7a1fe60f65670a51943605ee5b6961c1fdb88e661f5550";
    let (send, _recv) = init_send_recv();
    let mut txn = create_transfer_txn_default();

    // sign with txn+wallet method
    let txn_sig_arr = txn.sign(&send.wallet).serialize_compact();

    // get signature
    let secp = Secp256k1::new();
    let msg = &Message::from_slice(txn.calc_id().as_bytes()).unwrap();
    let sk = &send.kp.secret_key();
    let test_sig_secp = secp.sign_ecdsa(msg, sk);

    // as Signature
    let answer_secp = secp256k1::ecdsa::Signature::from_str(answer_str).unwrap();

    assert_eq!(answer_secp, test_sig_secp, "{test_sig_secp:?}");

    // as array
    let answer_arr = answer_secp.serialize_compact();
    assert_eq!(answer_arr, txn_sig_arr, "{txn_sig_arr:?}");

    // using the wallet fxn
    // assert!(
    //     Wallet::validate_txn_signature(&txn, &answer_secp, &send.pbkey()),
    //     "{test_sig_secp:?}"
    // );
    // assert!(
    //     Wallet::validate_txn_signature(&txn, &test_sig_secp, &send.pbkey()),
    //     "{test_sig_secp:?}"
    // );
}
