// imports
use std::net::IpAddr;
// local
use super::node::{
    error::{NodeError, P2PError},
    p2p::P2P,
};
use crate::ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet};
pub mod error;
pub mod p2p;
// submodule
pub type Result<T> = std::result::Result<T, NodeError>;

/// ## An instance of a `Node`.
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
    /// ## Create new instance of Node.
    ///
    /// Internal values will be updated separately, because:
    /// 1. There may be multiple ways to initialize a value (i.e. specific to a situation);
    /// 1. Values may be updated asynchronously;
    /// 1. It may not be necessary to initialize immediately.
    pub fn new() -> Self {
        Self {
            wallet: None,
            blockchain: None,
            txn_pool: None,
            p2p: None,
        }
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////
    /// ## Get ref of wallet for this instance of the node.
    pub fn wallet(&self) -> Result<&Wallet> {
        match &self.wallet {
            Some(w) => Ok(&w),
            None => Err(NodeError::InitWallet),
        }
    }
    /// ## Get ref of blockchain for this instance of the node.
    pub fn blockchain(&self) -> Result<&Blockchain> {
        match &self.blockchain {
            Some(b) => Ok(&b),
            None => Err(NodeError::InitBlockchain),
        }
    }
    /// ## Get ref of transaction pool for this instance of the node.
    pub fn txn_pool(&self) -> Result<&TxnPool> {
        match &self.txn_pool {
            Some(t) => Ok(&t),
            None => Err(NodeError::InitTxnPool),
        }
    }
    /// ## Get ref of peer to peer connection information for this instance of the node.
    pub fn p2p(&self) -> Result<&P2P> {
        match &self.p2p {
            Some(p) => Ok(&p),
            None => Err(NodeError::P2PError(P2PError::InitP2P)),
        }
    }
    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////
    /// ## Set the wallet for the node.
    ///
    /// Using a dedicated setter to standardize how the wallet gets updated.
    ///
    /// There will be more methods for initializing a wallet in the future.
    fn set_wallet(&mut self, wallet: Wallet) -> Result<()> {
        self.wallet = Some(wallet);

        Ok(())
    }
    /// ## Set the wallet for the node with a filepath.
    pub fn set_wallet_from_filepath(&mut self, filepath: &String) -> Result<()> {
        let wallet = Wallet::new_from_file(filepath);

        Ok(self.set_wallet(wallet)?)
    }
    /// ## INCOMPLETE Initialize the blockchain pulled from the peer to peer network.
    ///
    /// 1. Connect to p2p network
    /// 1. Check with other nodes
    /// 1. Fetch the hash of the blockchain
    /// 1. Update blockchain if hash mismatch (get missing blocks)
    /// Another setter for blockchain would be `set_blockchain_sync`
    pub fn set_blockchain_init(&mut self) -> Result<()> {
        todo!();
        // self.blockchain = Some(Blockchain::new());

        // Ok(())
    }
    /// ## INCOMPLETE Initialize the transaction pool pulled from the peer to peer network.
    ///
    /// 1. Connect to p2p network
    /// 1. Check with other nodes
    /// 1. Fetch the hash of the txn pool
    /// 1. Update txn pool if hash mismatch (get missing txns)
    /// Another setter for blockchain would be `set_txn_pool_sync`
    pub fn set_txn_pool_init(&mut self) -> Result<()> {
        todo!();
        // self.blockchain = Some(Blockchain::new());

        // Ok(())
    }
    /// ## Set p2p module
    pub fn set_p2p(&mut self, host: IpAddr, port: u16) -> Result<()> {
        self.p2p = Some(P2P::new(host, port));

        Ok(())
    }
    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// ACTIONS //////////////////////////////
    /// ## Start the p2p connection.
    pub fn start_p2p(&mut self) -> Result<()> {
        // check if p2p is initialized
        let p2p = self.p2p()?;

        // start the connection
        p2p.start_connection()?;

        Ok(())
    }
    ////////////////////////////// ACTIONS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}
