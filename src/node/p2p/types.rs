use crate::node::p2p::error::P2PError;

pub type Result<T> = std::result::Result<T, P2PError>;
