use super::general::PbKey;
use super::txn::{Txn, TxnHash};

// TODO: create BlockBody, house inside of `Block`
struct Block {
    pub transactions: Vec<Txn>,
    pub prev_blockhash: TxnHash,
    pub leader: PbKey,
    /// block count
    pub block_ct: u128,
}

impl Block {
    /// Create a new `Block` instance - not genesis
    /// TODO: Should check if genesis or not?
    /// transactions: List of transactions (`Txn`) to be included in the block
    pub fn new(transactions: Vec<Txn>, prev_blockhash: TxnHash, leader: PbKey) -> Self {
        Self {
            transactions,
            prev_blockhash,
            leader,
            block_ct: 0,
        }
    }
}

// tests:
// create a block
// add txn to block
// add block to chain
