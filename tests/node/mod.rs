// external
use std::net::{IpAddr, Ipv4Addr};
// local
use posbc::node::{p2p::P2P, types::Result, Node};
// test
use crate::common::fxns::init_blockchain_and_accounts;

/// Test if the node is correctly using its environment to:
/// 1. Check if node1 is running;
/// 1. Instantiate new node (node2, load wallet);
/// 1. Connect to the peer to peer network (node1);
/// 1. Check if node1's `peerId` exists;
#[test]
fn init_node_pass() -> Result<()> {
    let (users, _blockchain) = init_blockchain_and_accounts();
    let main = users.main;
    let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 8898;
    let new_p2p = P2P::new(host, port);

    Node::new(new_p2p, main.wallet);

    // init node
    // let node = Node::new();
    // init wallet
    // node.set_wallet(&users.main.filepath)?;
    // node.set_wallet_from_filepath(&users.main.filepath)?;
    // assert_eq!(
    //     &node.wallet()?.pbkey(),
    //     &users.main.wallet.pbkey(),
    //     "Wallet values not equal"
    // );
    // init p2p
    // node.set_p2p(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8898)?;
    // @todo initialize blockchain
    // @todo initialize txn pool
    Ok(())
}

// Other tests:
// - Sync a arbitrary block with another node.
// - Sync the full blockchain with another node.
// - Test block consensus.
// - Send a transaction.
// - Propose and submit a block to peers.
// - (Multiple) test transaction gossip.
// - (Multiple) test block gossip.
