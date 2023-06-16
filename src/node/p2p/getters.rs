// external
use std::net::{SocketAddr, TcpListener};
// local
use super::P2P;

impl P2P {
    /// ### Get property `P2P.socket_addr`.
    /// Default error behavior is to panic.
    pub fn socket_addr(&self) -> &SocketAddr {
        &self.socket_addr
    }
    /// ### Get property `P2P.listener`.
    /// Default error behavior is to panic.
    pub fn listener(&self) -> &TcpListener {
        &self.listener
    }
}
