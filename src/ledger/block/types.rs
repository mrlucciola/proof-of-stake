use crate::ledger::{
    block::constants::{BLOCK_DIGEST_LEN, BLOCK_SIGNATURE_CTX},
    txn_pool::TxnMap,
};

/// ### This is TxnMap with added functionality.
/// @todo add condition that this map cant have more than _ number of txns.
pub type BlockTxnMap = TxnMap;

pub type BlockDigest = [u8; 64];
pub type BlockCtxDigest = [u8; BLOCK_SIGNATURE_CTX.len() + BLOCK_DIGEST_LEN];
