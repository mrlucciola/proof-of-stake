pub mod error;
mod getters;
mod types;
mod utils;
// external
use libp2p::PeerId;
use std::net::{IpAddr, SocketAddr, TcpListener};
// local
use crate::ledger::general::PbKey;
pub use error::P2PError;

/// ## Peer-to-peer connection.
#[derive(Debug)]
pub struct P2P {
    socket_addr: SocketAddr,
    listener: TcpListener,
    peer_id: PeerId,
}

impl P2P {
    /// ### Initialize a new peer-to-peer connection
    /// Get environment information from a config file.
    pub fn new(host: IpAddr, port: u16, peer_pubkey: &PbKey) -> Self {
        let socket_addr = SocketAddr::new(host, port);
        Self {
            socket_addr,
            listener: TcpListener::bind(socket_addr).unwrap(),
            peer_id: PeerId::from_public_key(&peer_pubkey.to_owned().into()),
        }
    }
}
