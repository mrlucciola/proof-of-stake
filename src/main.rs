// imports
// local
pub mod ledger;
// use ledger::*;

use ledger::txn::create_sample_txn;

use crate::ledger::wallet::Wallet;
// use crate::ledger::txn::create_sample_txn;

fn main() {
    let txn = create_sample_txn();
    println!("{txn:#?}");

    let wallet = Wallet::new();
    let signature = txn.sign(wallet);
}
