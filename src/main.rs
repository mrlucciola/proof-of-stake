fn main() {
    println!("Hello, world!");
}

struct Txn<'a> {
    sender_pubkey: &'a String,
    recv_pubkey: &'a String,
    amt: u128,
    txn_type: TxnType,
}
enum TxnType {
    Transfer,
}

impl<'a> Txn<'a> {
    fn new(
        &self,
        sender_pubkey: &'a String,
        recv_pubkey: &'a String,
        amt: u128,
        txn_type: TxnType,
    ) -> Self {
        Self {
            sender_pubkey,
            recv_pubkey,
            amt,
            txn_type,
        }
    }
}
