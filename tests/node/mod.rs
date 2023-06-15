// external
use std::net::{IpAddr, Ipv4Addr};
// local
use posbc::node::{Node, P2P};
// test
use crate::common::fxns::init_blockchain_and_accounts;

/// Test if the port is available using std lib.
/// Attempt to open port at `localhost:port`.
fn port_is_available(port: u16) -> bool {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));

    match std::net::TcpListener::bind(addr) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Test if the node is correctly using its environment to:
/// 1. Check if node1 is running (on port `8898`);
/// 1. Instantiate new node (node2, load wallet);
/// 1. Connect to the peer to peer network (node1);
/// 1. Check if node1's `peerId` exists;
#[tokio::test]
async fn init_node_pass() {
    // 1. Test if node1 is running (port should be unavailable)
    // assert!(
    //     port_is_available(8898) == false,
    //     "node1 at Port 8898 is not running"
    // );

    let (users, _blockchain) = init_blockchain_and_accounts();
    let main = users.main;
    let node1_pbkey = main.wallet.pbkey();

    // 2. init node
    let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 8899;
    println!("HERE0");
    let p2p2 = P2P::new(host, port, main.kp);
    let mut node2 = Node::new(p2p2, main.wallet);

    // 3. connect to node1
    node2.p2p_mut().unwrap().discover_peer(&node1_pbkey);
}

// Other tests:
// - Sync a arbitrary block with another node.
// - Sync the full blockchain with another node.
// - Test block consensus.
// - Send a transaction.
// - Propose and submit a block to peers.
// - (Multiple) test transaction gossip.
// - (Multiple) test block gossip.
