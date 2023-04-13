// external
use std::net::{IpAddr, SocketAddr, TcpListener};
// local
use super::P2P;

impl P2P {
    /// Get `P2P.host` property.
    pub fn get_host(&self) -> &IpAddr {
        &self.host
    }
    /// Get `P2P.port` property.
    pub fn get_port(&self) -> &u16 {
        &self.port
    }
    /// Get `P2P.socket_addr` property.
    pub fn get_socket_addr(&self) -> &SocketAddr {
        &self.socket_addr
    }
    /// Get `P2P.listener` property.
    pub fn get_listener(&self) -> &TcpListener {
        &self.listener
    }
}
