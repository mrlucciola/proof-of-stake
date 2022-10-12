// imports
// local
pub mod ledger;
// use ledger::*;

use ledger::txn::create_sample_unsigned_txn;

use crate::ledger::wallet::Wallet;
// use crate::ledger::txn::create_sample_txn;

fn main() {
    let mut txn = create_sample_unsigned_txn();
    println!("{txn:#?}");

    let mut wallet = Wallet::new(&"test_key.json".to_string());
    txn.sign(&wallet);
}
