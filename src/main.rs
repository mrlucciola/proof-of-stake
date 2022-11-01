// imports
use anyhow::Result;
// local
pub mod ledger;
use crate::ledger::wallet::Wallet;
use ledger::txn::create_sample_unsigned_txn;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut txn = create_sample_unsigned_txn();
    println!("{txn:#?}");

    // get signature
    let wallet = Wallet::new(&"test_key.json".to_string());
    txn.sign(&wallet)?;

    let signature = txn.signature.unwrap();
    println!("signature: {signature:?}");
    // println!("valid signature: {:?}", txn.validate_signature(&wallet));

    Ok(())
}
