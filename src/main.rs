// imports
use anyhow::Result;
use ledger::txn::{Txn, TxnType};
use secp256k1::ecdsa::Signature as TxnSignature;
// local
pub mod ledger;
use crate::ledger::wallet::Wallet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signature = example_create_and_sign_txn();
    println!("signature: {signature:?}");
    // println!("valid signature: {:?}", txn.validate_signature(&wallet));

    Ok(())
}

fn example_create_and_sign_txn() -> (Txn, TxnSignature) {
    let wallet_main = Wallet::new_from_file(&"test_key.json".to_string());
    let wallet_recv = Wallet::new_from_file(&"test_key_recv.json".to_string());

    // turn the raw txn into message
    let sender_pbkey = wallet_main.get_pubkey();
    let recv_pbkey = wallet_recv.get_pubkey();
    let amt = 100;
    let txn_type = TxnType::Transfer;
    let mut txn: Txn = Txn::new(sender_pbkey, recv_pbkey, amt, txn_type);

    // get signature
    let signature: TxnSignature = wallet_main.sign(&mut txn);
    // txn.signature = Some(signature);

    (txn, signature)
}
