// imports
use secp256k1::{Message, Secp256k1};
use std::str::FromStr;
// local
use posbc::ledger::{general::SecpEcdsaSignature, txn::Txn, wallet::Wallet};
pub mod common;
use common::{create_transfer_txn, init_send_recv};

#[test]
fn verify_signature_pass() {
    // init
    let answer_str = "30440220136c9d8e942c527262695b2ceeb2b90ff7f7650e9230a17355dcc560ea3c7648022029e1519209a08d3a2a9d8d8b55e39c07d739bb1780329d09d0273a07695a9fa9";
    let (send, _recv) = init_send_recv();
    let mut txn = create_transfer_txn();

    // sign with txn+wallet method
    let txn_sig_arr = txn.sign(&send.wallet);
    // convert to Signature
    // let txn_sig_secp = SecpEcdsaSignature::from_compact(&txn_sig_arr).unwrap();

    // get signature
    let secp = Secp256k1::new();
    // let test_sig_secp: SecpEcdsaSignature = secp.sign_ecdsa(
    //     &Message::from_slice(id_abstract.as_bytes()).unwrap(),
    //     &send.kp.secret_key(),
    // );

    // as Signature
    // let answer_secp = SecpEcdsaSignature::from_str(answer_str).unwrap();

    // assert_eq!(answer_secp, test_sig_secp, "{test_sig_secp:?}");
    // assert_eq!(txn_sig_secp, test_sig_secp, "{test_sig_secp:?}");
    // as array
    // let answer_arr = answer_secp.serialize_compact();
    // assert_eq!(answer_arr, txn_sig_arr, "{txn_sig_arr:?}");

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
