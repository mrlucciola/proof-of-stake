// imports
use anyhow::Result;
// local
pub mod ledger;
use crate::ledger::wallet::Wallet;
use ledger::txn::create_sample_unsigned_txn;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut txn = create_sample_unsigned_txn();
    println!("{txn:#?}");

    let wallet = Wallet::new(&"test_key.json".to_string());
    txn.sign(&wallet)?;
    println!("valid signature: {:?}", txn.validate_signature(&wallet));

    Ok(())
}
