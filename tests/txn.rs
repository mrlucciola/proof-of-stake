// imports
use secp256k1::{Message, Secp256k1};
// local
use posbc::ledger::{general::SecpEcdsaSignature, txn::Txn};
pub mod common;
use common::{create_transfer_txn, init_send_recv};

#[test]
fn create_unsigned_txn_pass() {
    let answer = "52b25d9d1ddf85eeaba1874069fd00236470a74ad7ed8a695bfcb68fd523e851";
    let txn = create_transfer_txn();
    let id_abstract = Txn::get_id(&txn);
    // let msg = msg.as_slice();
    let msg = id_abstract.to_string();

    assert!(msg == answer, "{:?}", msg);
}

#[test]
fn create_signed_txn_pass() {
    let (send, _recv) = init_send_recv();
    let txn = create_transfer_txn();
    let id = Txn::get_id(&txn);

    // get signature
    let secp = Secp256k1::new();
    let signature: SecpEcdsaSignature = secp.sign_ecdsa(
        &Message::from_slice(id.as_bytes()).unwrap(),
        &send.kp.secret_key(),
    );
    let answer = "30440220136c9d8e942c527262695b2ceeb2b90ff7f7650e9230a17355dcc560ea3c7648022029e1519209a08d3a2a9d8d8b55e39c07d739bb1780329d09d0273a07695a9fa9";

    assert!(signature.to_string() == answer.to_string(), "{signature:?}");
}
