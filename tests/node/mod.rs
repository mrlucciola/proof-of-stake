// imports
use std::net::{IpAddr, Ipv4Addr};
// local
use posbc::node::{Node, Result, P2P};
// test
use crate::common::fxns::init_blockchain_and_accounts;

/// Test if the node is correctly using its environment to:
/// 1. Connect to the peer to peer network
/// 1. Load wallet correctly
/// 1. Initialize its values correctly (blockchain, txn pool, wallet)
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
