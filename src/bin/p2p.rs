use std::net::{IpAddr, Ipv4Addr};

// imports
// local
use posbc::{ledger::general::Result, node::Node};

fn main() -> Result<()> {
    let mut node = Node::new();
    let kp_filepath = String::from("tests/keys/main.json");
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port: u16 = 8898;

    node.set_wallet_from_filepath(&kp_filepath)?;
    node.set_p2p(ip, port)?;
    node.start_p2p()?;

    Ok(())
}
