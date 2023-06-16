use crate::{
    ledger::general::{PbKey, KP},
    node::p2p::P2P,
};
use {libp2p::PeerId, std::net::SocketAddr};

impl P2P {
    /// ### Get property `P2P.socket_addr`.
    /// Default error behavior is to panic.
    pub fn socket_addr(&self) -> &SocketAddr {
        &self.socket_addr
    }
    fn kp(&self) -> &KP {
        &self.kp
    }
    /// ### Get property `P2P.pbkey`.
    pub fn pbkey(&self) -> PbKey {
        self.kp().public.into()
    }
    /// ### Get property `P2P.peer_id`.
    pub fn peer_id(&self) -> PeerId {
        PeerId::from_public_key(&self.pbkey().to_owned().into())
    }
}
