use crate::node::error::NodeError;

pub type Result<T> = std::result::Result<T, NodeError>;
