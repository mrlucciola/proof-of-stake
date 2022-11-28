// imports
use anyhow::Result;
use ledger::txn::{Txn, TxnSig, TxnType};
// local
pub mod ledger;
use crate::ledger::wallet::Wallet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signature = example_create_and_sign_txn();
    println!("signature: {signature:?}");

    Ok(())
}

fn example_create_and_sign_txn() -> (Txn, TxnSig) {
    let wallet_main = Wallet::new_from_file(&"test_key.json".to_string());
    let wallet_recv = Wallet::new_from_file(&"test_key_recv.json".to_string());

    // turn the raw txn into message
    let sender_pbkey = wallet_main.get_pbkey();
    let recv_pbkey = wallet_recv.get_pbkey();
    let amt = 100;
    let txn_type = TxnType::Transfer;
    let txn: Txn = Txn::new(sender_pbkey, recv_pbkey, amt, txn_type);

    // get signature
    let signature: TxnSig = wallet_main.get_signature(&txn.hash());

    (txn, signature)
}
