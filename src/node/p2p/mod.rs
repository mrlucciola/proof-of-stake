pub mod error;
mod getters;
mod types;
mod utils;
// external
use std::net::{IpAddr, SocketAddr, TcpListener};
// local
pub use error::P2PError;

#[derive(Debug)]
pub struct P2P {
    pub host: IpAddr,
    pub port: u16,
    pub socket_addr: SocketAddr,
    listener: TcpListener,
}

impl P2P {
    /// ### Initialize a new peer-to-peer connection
    /// Get environment information from a config file.
    pub fn new(host: IpAddr, port: u16) -> Self {
        let socket_addr = SocketAddr::new(host, port);
        let listener = TcpListener::bind(socket_addr).unwrap();

        Self {
            host,
            port,
            socket_addr,
            listener,
        }
    }
}
