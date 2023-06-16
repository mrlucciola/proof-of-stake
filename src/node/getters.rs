use crate::{
    ledger::{blockchain::Blockchain, txn_pool::TxnPool, wallet::Wallet},
    node::{p2p::P2P, Node},
};
use std::borrow::BorrowMut;

impl Node {
    /// ### Get property `Node.wallet`.
    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }
    /// ### Get property `Node.blockchain`.
    /// Default error behavior is to return error if property is not set.
    pub fn blockchain(&self) -> &Blockchain {
        &self.blockchain
    }
    /// ### Get property `Node.transaction_pool`.
    /// Default error behavior is to return error i property is not set.
    pub fn txn_pool(&self) -> &TxnPool {
        &self.txn_pool
    }
    /// ### Get property `Node.p2p`.
    /// Node.p2p is the node's peer to peer instance.
    pub fn p2p(&self) -> &P2P {
        &self.p2p
    }
    /// ### Get property `Node.p2p` as mutable.
    /// Node.p2p is the node's peer to peer instance.
    pub fn p2p_mut(&mut self) -> &mut P2P {
        self.p2p.borrow_mut()
    }
}
