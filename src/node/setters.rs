// external
use std::net::IpAddr;
// local
pub use super::{types::Result, Node, P2P};

impl Node {
    //////////////////////////////////////////////////
    ////////////////// INITIALIZERS //////////////////
    #[deprecated = "Not implemented"]
    #[allow(dead_code)]
    fn init_p2p(&mut self, host: IpAddr, port: u16) -> Result<()> {
        let new_p2p = P2P::new(host, port);
        self.set_p2p(new_p2p)?;

        Ok(())
    }

    /// ### INCOMPLETE Initialize the blockchain pulled from the peer to peer network.
    ///
    /// 1. Connect to p2p network
    /// 1. Check with other nodes
    /// 1. Fetch the hash of the blockchain
    /// 1. Update blockchain if hash mismatch (get missing blocks)
    /// Another setter for blockchain would be `set_blockchain_sync`
    #[deprecated = "Not implemented"]
    pub fn init_blockchain(&mut self) -> Result<()> {
        todo!();
        // self.blockchain = Some(Blockchain::new());

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
    pub fn init_txn_pool(&mut self) -> Result<()> {
        todo!();
        // self.blockchain = Some(Blockchain::new());

        // Ok(())
    }
    ////////////////// INITIALIZERS //////////////////
    //////////////////////////////////////////////////
}
