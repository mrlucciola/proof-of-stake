// imports
use secp256k1::{Message, Secp256k1};
// local
use posbc::ledger::{general::SecpEcdsaSignature, txn::Txn};
pub mod common;
use common::{create_transfer_txn, init_send_recv};

#[test]
fn create_unsigned_txn_pass() {
    let answer = "730a7444fcd3b916537d381d39c07e19201d48e09949a50ae84347767af92bfd";
    let txn = create_transfer_txn();
    let hash = Txn::get_hash(&txn);
    // let msg = msg.as_slice();
    let msg = hash.to_string();

    assert!(msg == answer, "{:?}", msg);
}

#[test]
fn create_signed_txn_pass() {
    let (send, _recv) = init_send_recv();
    let txn = create_transfer_txn();
    let hash = Txn::get_hash(&txn);

    // get signature
    let secp = Secp256k1::new();
    let signature: SecpEcdsaSignature = secp.sign_ecdsa(
        &Message::from_slice(hash.as_bytes()).unwrap(),
        &send.kp.secret_key(),
    );
    let answer = "3044022038d2bcb6824e0f32e725f258494f71ac1963f83dce802f2f16b40613f35ffdbc022030fa63f8dd11422c8535688c67fa28c8214e96bc91c5586c03ec7685f65d6ff5";

    assert!(signature.to_string() == answer.to_string(), "{signature:?}");
}
