// local
use super::{types::Result, Node};
use crate::ledger::wallet::Wallet;

impl Node {
    /// ### Start the p2p connection.
    pub fn start_p2p(&mut self) -> Result<()> {
        // check if p2p is initialized
        let p2p_module = self.p2p()?;
        // start the connection
        p2p_module.start_connection()?;

        Ok(())
    }
    /// ### Set the wallet for the node with a filepath.
    /// Standardized setter for loading wallet from keypair file.
    /// There will be more methods for initializing a wallet in the future.
    /// - Parse config file
    /// - Get filepath for keypair from config
    /// -
    pub fn get_wallet_from_filepath(filepath: Option<&String>) -> Result<Wallet> {
        let wallet_filepath = match filepath {
            // @todo validation logic
            Some(fp) => fp,
            None => "hidden/master_key_ed25519.json",
        }
        .to_string();
        let wallet = Wallet::new_from_file(&wallet_filepath);

        Ok(wallet)
    }
}
