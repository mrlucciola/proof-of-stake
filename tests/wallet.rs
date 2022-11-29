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
    let answer_str = "3044022038d2bcb6824e0f32e725f258494f71ac1963f83dce802f2f16b40613f35ffdbc022030fa63f8dd11422c8535688c67fa28c8214e96bc91c5586c03ec7685f65d6ff5";
    let (send, _recv) = init_send_recv();
    let mut txn = create_transfer_txn();

    let hash = Txn::get_hash(&txn);
    // sign with txn+wallet method
    let txn_sig_arr = txn.sign(&send.wallet);
    // convert to Signature
    let txn_sig_secp = SecpEcdsaSignature::from_compact(&txn_sig_arr).unwrap();

    // get signature
    let secp = Secp256k1::new();
    let test_sig_secp: SecpEcdsaSignature = secp.sign_ecdsa(
        &Message::from_slice(hash.as_bytes()).unwrap(),
        &send.kp.secret_key(),
    );

    // as Signature
    let answer_secp = SecpEcdsaSignature::from_str(answer_str).unwrap();

    assert_eq!(answer_secp, test_sig_secp, "{test_sig_secp:?}");
    assert_eq!(txn_sig_secp, test_sig_secp, "{test_sig_secp:?}");
    // as array
    let answer_arr = answer_secp.serialize_compact();
    assert_eq!(answer_arr, txn_sig_arr, "{txn_sig_arr:?}");

    // using the wallet fxn
    assert!(
        Wallet::validate_signature(&txn, &answer_secp, &send.pbkey()),
        "{test_sig_secp:?}"
    );
    assert!(
        Wallet::validate_signature(&txn, &test_sig_secp, &send.pbkey()),
        "{test_sig_secp:?}"
    );
}
