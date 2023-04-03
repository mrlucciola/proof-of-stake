// local
use posbc::node::{Node, Result};
// test
use crate::common::fxns::init_blockchain_and_accounts;

/// Test if the node is correctly using its environment to:
/// 1. Connect to the peer to peer network
/// 1. Load wallet correctly
/// 1. Initialize its values correctly (blockchain, txn pool, wallet)
#[test]
fn init_node_pass() -> Result<()> {
    let (users, _blockchain) = init_blockchain_and_accounts();

    // initialize node
    let mut node = Node::new();
    // initialize wallet
    node.set_wallet_from_filepath(&users.main.filepath)?;
    assert_eq!(
        &node.wallet()?.pbkey(),
        &users.main.wallet.pbkey(),
        "Wallet values not equal"
    );

    // @todo initialize p2p
    // @todo initialize blockchain
    // @todo initialize txn pool
    Ok(())
}

// #[test]
// #[should_panic]
// fn init_node_fail() {
//     let node = Node::new();
// }
