// local
use super::{Node, Result};

impl Node {
    /// ### Start the p2p connection.
    pub fn start_p2p(&mut self) -> Result<()> {
        // check if p2p is initialized
        let p2p_module = self.p2p()?;
        // start the connection
        p2p_module.start_connection()?;

        Ok(())
    }
}
