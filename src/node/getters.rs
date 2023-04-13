// local
use super::{
    error::{NodeError, P2PError},
    p2p::P2P,
    Node, Result,
};
use crate::ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet};

impl Node {
    /// ### Get `Node.wallet` property.
    ///
    /// Default error behavior is to return error if property is not set.
    pub fn wallet(&self) -> Result<&Wallet> {
        match &self.wallet {
            Some(w) => Ok(&w),
            None => Err(NodeError::InitWallet),
        }
    }
    /// ### Get `Node.blockchain` property.
    ///
    /// Default error behavior is to return error if property is not set.
    pub fn blockchain(&self) -> Result<&Blockchain> {
        match &self.blockchain {
            Some(b) => Ok(&b),
            None => Err(NodeError::InitBlockchain),
        }
    }
    /// ### Get `Node.transaction_pool` property.
    ///
    /// Default error behavior is to return error if property is not set.
    pub fn txn_pool(&self) -> Result<&TxnPool> {
        match &self.txn_pool {
            Some(t) => Ok(&t),
            None => Err(NodeError::InitTxnPool),
        }
    }
    /// ### Get `Node.p2p` property.
    /// Node.p2p is the node's peer to peer instance.
    ///
    /// Default error behavior is to return error if property is not set.
    pub fn p2p(&self) -> Result<&P2P> {
        match &self.p2p {
            Some(p) => Ok(&p),
            None => Err(NodeError::P2PError(P2PError::InitP2P)),
        }
    }
    pub fn p2p_mut(&mut self) -> Result<&P2P> {
        match &self.p2p {
            Some(p) => Ok(&p),
            None => Err(NodeError::P2PError(P2PError::InitP2P)),
        }
    }
}