// imports
// local
use crate::ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet};
pub mod error;
use crate::node::error::NodeError;
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
}

impl Node {
    /// ## Create new instance of Node.
    ///
    /// Internal values will be updated separately, because:
    /// 1. There may be multiple ways to initialize a value (i.e. specific to a situation);
    /// 1. Values may be updated asynchronously;
    /// 1. It may not be necessary to initialize immediately.
    pub fn new() -> Self {
        let wallet = None;
        let blockchain = None;
        let txn_pool = None;

        Self {
            wallet,
            blockchain,
            txn_pool,
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
    ////////////////////////////// SETTERS //////////////////////////////
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
