// imports
// local
pub mod txn;
pub use txn::*;

fn main() {
    let txn = create_sample_txn();
    println!("{txn:#?}");
}
