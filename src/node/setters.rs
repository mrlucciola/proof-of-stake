// local
use crate::node::{types::Result, Node};

impl Node {
    //////////////////////////////////////////////////
    ////////////////// INITIALIZERS //////////////////

    /// ### INCOMPLETE Initialize the blockchain pulled from the peer to peer network.
    ///
    /// 1. Connect to p2p network
    /// 1. Check with other nodes
    /// 1. Fetch the hash of the blockchain
    /// 1. Update blockchain if hash mismatch (get missing blocks)
    /// Another setter for blockchain would be `set_blockchain_sync`
    #[deprecated = "Not implemented"]
    pub fn sync_blockchain(&mut self) -> Result<()> {
        todo!();
        // Ok(())
    }

    /// ### INCOMPLETE Initialize the transaction pool pulled from the peer to peer network.
    ///
    /// 1. Connect to p2p network
    /// 1. Check with other nodes
    /// 1. Fetch the hash of the txn pool
    /// 1. Update txn pool if hash mismatch (get missing txns)
    /// Another setter for blockchain would be `set_txn_pool_sync`
    #[deprecated = "Not implemented"]
    pub fn sync_txn_pool(&mut self) -> Result<()> {
        todo!();
        // Ok(())
    }
    ////////////////// INITIALIZERS //////////////////
    //////////////////////////////////////////////////
}
