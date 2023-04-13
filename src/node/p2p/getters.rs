// external
use std::net::{SocketAddr, TcpListener};
// local
use super::P2P;

impl P2P {
    /// ### Get `P2P.socket_addr` property.
    /// Default error behavior is to panic.
    pub fn socket_addr(&self) -> &SocketAddr {
        self.socket_addr.as_ref().unwrap()
    }
    /// ### Get `P2P.listener` property.
    /// Default error behavior is to panic.
    pub fn listener(&self) -> &TcpListener {
        self.listener.as_ref().unwrap()
    }
}
