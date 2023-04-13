mod error;
mod getters;
mod p2p;
mod setters;
mod types;
mod utils;
// local
use crate::ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet};
pub use {error::NodeError, p2p::P2P, types::Result};

/// ### An instance of a `Node`.
/// A node has a wallet, an instance of blockchain, and a transaction pool.
/// Node will sync its blockchain ledger and transaction pool with its peers
/// via the peer-to-peer network (p2p).
///
/// The struct is set up this way to:
/// 1. Prioritize modularity;
/// 1. Test components in isolation;
/// 1. Decouple logic wherever possible;
/// 1. Leverage the type-system for error handling;
#[derive(Debug)]
pub struct Node {
    wallet: Option<Wallet>,
    blockchain: Option<Blockchain>,
    txn_pool: Option<TxnPool>,
    p2p: Option<P2P>,
}

impl Node {
    /// ### Create a new `Node` instance.
    ///
    /// blockchain and txn pool values will be updated separately, because:
    /// 1. There may be multiple ways to initialize them (i.e. specific to a situation);
    /// 1. They may be updated asynchronously;
    /// 1. It may not be necessary to initialize immediately.
    pub fn new(p2p: P2P, wallet: Wallet) -> Self {
        let mut new_node = Self {
            wallet: None,
            blockchain: None,
            txn_pool: None,
            p2p: None,
        };

        new_node.set_p2p(p2p).unwrap();
        new_node.set_wallet(wallet).unwrap();

        new_node
    }

    /////////////////////////////////////////////////////
    ////////////////// PROPERTY SETTERS /////////////////
    /// ### Set `Node.wallet` property with `Wallet` instance.
    fn set_wallet(&mut self, wallet: Wallet) -> Result<()> {
        self.wallet = Some(wallet);

        Ok(())
    }
    /// ### Set `Node.blockchain` property with `P2P` instance.
    #[allow(dead_code)]
    fn set_blockchain(&mut self, blockchain: Blockchain) -> Result<()> {
        self.blockchain = Some(blockchain);

        Ok(())
    }
    /// ### Set `Node.txn_pool` property with `TxnPool` instance.
    #[allow(dead_code)]
    fn set_txn_pool(&mut self, txn_pool: TxnPool) -> Result<()> {
        self.txn_pool = Some(txn_pool);

        Ok(())
    }
    /// ### Set `Node.p2p` property with `P2P` instance.
    fn set_p2p(&mut self, p2p: P2P) -> Result<()> {
        self.p2p = Some(p2p);

        Ok(())
    }
    ////////////////// PROPERTY SETTERS /////////////////
    /////////////////////////////////////////////////////
}
