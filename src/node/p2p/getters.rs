// external
use std::net::{SocketAddr, TcpListener};
// local
use super::P2P;

impl P2P {
    /// Get `P2P.socket_addr` property.
    pub fn socket_addr(&self) -> &SocketAddr {
        &self.socket_addr
    }
    /// Get `P2P.listener` property.
    pub fn listener(&self) -> &TcpListener {
        &self.listener
    }
}
