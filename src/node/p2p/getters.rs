// external
use {libp2p::PeerId, std::net::SocketAddr};
// local
use crate::{ledger::general::PbKey, node::p2p::P2P};

impl P2P {
    /// ### Get property `P2P.socket_addr`.
    /// Default error behavior is to panic.
    pub fn socket_addr(&self) -> &SocketAddr {
        &self.socket_addr
    }
    /// ### Get `P2P.pbkey` property.
    pub fn pbkey(&self) -> &PbKey {
        &self.kp.public.into()
    }
    /// ### Get `P2P.peer_id` property.
    pub fn peer_id(&self) -> PeerId {
        PeerId::from_public_key(&self.pbkey().to_owned().into())
    }
}
