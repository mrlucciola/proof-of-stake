// local
use super::{
    error::NodeError,
    p2p::{P2PError, P2P},
    types::Result,
    Node,
};
use crate::ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet};

impl Node {
    /// ### Get `Node.wallet` property.
    /// Default error behavior is to return error if property is not set.
    pub fn wallet(&self) -> Result<&Wallet> {
        match self.wallet.as_ref() {
            Some(w) => Ok(w),
            None => Err(NodeError::InitWallet),
        }
    }
    /// ### Get `Node.blockchain` property.
    /// Default error behavior is to return error if property is not set.
    pub fn blockchain(&self) -> Result<&Blockchain> {
        match self.blockchain.as_ref() {
            Some(b) => Ok(b),
            None => Err(NodeError::InitBlockchain),
        }
    }
    /// ### Get `Node.transaction_pool` property.
    /// Default error behavior is to return error if property is not set.
    pub fn txn_pool(&self) -> Result<&TxnPool> {
        match self.txn_pool.as_ref() {
            Some(t) => Ok(t),
            None => Err(NodeError::InitTxnPool),
        }
    }
    /// ### Get `Node.p2p` property.
    /// Node.p2p is the node's peer to peer instance.\
    /// Default error behavior is to return error if property is not set.
    pub fn p2p(&self) -> Result<&P2P> {
        match self.p2p.as_ref() {
            Some(p) => Ok(p),
            None => Err(NodeError::P2PError(P2PError::InitP2P)),
        }
    }
    /// ### Get `Node.p2p` property as a mutable ref.
    pub fn p2p_mut(&mut self) -> Result<&P2P> {
        match self.p2p.as_mut() {
            Some(p) => Ok(p),
            None => Err(NodeError::P2PError(P2PError::InitP2P)),
        }
    }
}
