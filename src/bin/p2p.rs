use std::net::{IpAddr, Ipv4Addr};

// imports
// local
use posbc::{
    ledger::general::Result,
    node::{Node, P2P},
};

fn main() -> Result<()> {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port: u16 = 8898;
    let new_p2p = P2P::new(ip, port);
    let kp_filepath = String::from("tests/keys/main.json");
    let new_wallet = Node::get_wallet_from_filepath(Some(&kp_filepath)).unwrap();

    let mut node = Node::new(new_p2p, new_wallet);
    node.start_p2p()?;

    Ok(())
}
