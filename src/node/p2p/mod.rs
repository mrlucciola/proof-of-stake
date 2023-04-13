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
    socket_addr: Option<SocketAddr>,
    listener: Option<TcpListener>,
}

impl P2P {
    /// ### Initialize a new peer-to-peer connection
    /// Get environment information from a config file.
    pub fn new(host: IpAddr, port: u16) -> Self {
        let mut new_p2p = Self {
            socket_addr: None,
            listener: None,
        };

        new_p2p.set_socket_addr(host, port);

        new_p2p
    }

    /////////////////////////////////////////////////////
    ////////////////// PROPERTY SETTERS /////////////////
    /// Set `P2P.socket_addr` property.
    fn set_socket_addr(&mut self, new_host: IpAddr, new_port: u16) {
        self.socket_addr = Some(SocketAddr::new(new_host, new_port));
        self.set_listener();
    }
    /// Set `P2P.listener` property.
    fn set_listener(&mut self) {
        // drop current value
        self.listener = None;
        // set new one
        self.listener = Some(TcpListener::bind(self.socket_addr()).unwrap());
    }
    ////////////////// PROPERTY SETTERS /////////////////
    /////////////////////////////////////////////////////
}
