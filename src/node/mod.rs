mod error;
mod getters;
pub mod p2p;
mod setters;
pub mod types;
mod utils;

use crate::{
    ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet},
    node::p2p::P2P,
};

/// ## An instance of a `Node`.
/// A node has a wallet, an instance of blockchain, and a transaction pool.\
/// Node will sync its blockchain ledger and transaction pool with its peers\
/// via the peer-to-peer network (p2p).
///
/// The struct is set up this way to:
/// 1. Prioritize modularity;
/// 1. Test components in isolation;
/// 1. Decouple logic wherever possible;
/// 1. Leverage the type-system for error handling;
#[derive(Debug)]
pub struct Node {
    wallet: Wallet,
    blockchain: Blockchain,
    txn_pool: TxnPool,
    p2p: P2P,
}

// @todo store node info locally so that it can be retrieved on startup
impl Node {
    /// ### Create a new `Node` instance.
    ///
    /// blockchain and txn pool values will be updated separately, because:
    /// 1. There may be multiple ways to initialize them (i.e. specific to a situation);
    /// 1. They may be updated asynchronously;
    /// 1. It may not be necessary to initialize immediately.
    pub fn new(p2p: P2P, wallet: Wallet) -> Self {
        Self {
            wallet,
            blockchain: Blockchain::new(),
            txn_pool: TxnPool::new(),
            p2p,
        }
    }
}
