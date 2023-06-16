use crate::{
    ledger::wallet::Wallet,
    node::{types::Result, Node},
};

impl Node {
    /// ### Start the p2p connection.
    /// @todo call libp2p
    pub fn start_p2p(&mut self) -> Result<()> {
        // check if p2p is initialized
        let _p2p_module = self.p2p();
        // start the connection
        // p2p_module.start_connection()?;

        Ok(())
    }
    /// ### Set the wallet for the node with a filepath.
    /// Load a wallet from keypair file.\
    /// This is currently just a wrapper for Wallet::new_from_file().\
    /// There will be more methods for initializing a wallet in the future.
    /// - @todo fxn1: add config file and parse for filepath;
    /// - @todo fxn2: get filepath from stdin;
    pub fn get_wallet_from_filepath(filepath: &String) -> Result<Wallet> {
        // let wallet_filepath = match filepath {
        //     // @todo validation logic
        //     Some(fp) => fp,
        //     None => "hidden/master_key_ed25519.json",
        // }
        // .to_string();

        Ok(Wallet::new_from_file(filepath))
    }
}
