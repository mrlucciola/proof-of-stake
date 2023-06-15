use std::collections::BTreeMap;

// external
use serde::{Deserialize, Serialize};
// local
use crate::{
    ledger::{block::Block, blockchain::types::BlockMapKey},
    node::p2p::{types::Result, P2P},
};

impl P2P {
    /// ### Change to libp2p library
    pub fn start_connection(&self) -> Result<()> {
        Ok(())
    }
    /// ### Start the p2p connection.
    #[deprecated(note = "To be refactored for libp2p.")]
    pub fn start_p2p(&mut self) -> Result<()> {
        // start the connection
        // self.start_connection()?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Nodes(BTreeMap<BlockMapKey, Block>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub mode: ListMode,
    pub data: Nodes,
    pub receiver: String,
}
