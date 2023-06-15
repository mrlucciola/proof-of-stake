use std::{
    fs::File,
    io::BufReader,
    net::{IpAddr, Ipv4Addr},
};

// imports
// local
use posbc::{
    ledger::general::{Result, KP},
    node::{Node, P2P},
};

fn main() -> Result<()> {
    let kp_filepath = String::from("tests/keys/main_ed25519.json");
    let f = File::open(kp_filepath.clone()).unwrap();
    let reader = BufReader::new(f);
    let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();

    // open with ed 25519 lib
    let kp = KP::from_bytes(&key_json).unwrap();
    let new_wallet = Node::get_wallet_from_filepath(Some(&kp_filepath)).unwrap();

    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port: u16 = 8898;
    let new_p2p = P2P::new(ip, port, kp);

    let mut node = Node::new(new_p2p, new_wallet);
    node.start_p2p()?;

    Ok(())
}
