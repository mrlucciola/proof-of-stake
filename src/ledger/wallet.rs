// imports
use anyhow::format_err;
use ed25519_dalek::Sha512;
use secp256k1::rand::{rngs, SeedableRng};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
// local
use crate::{
    ledger::{
        blocks::Block,
        general::{PbKey, Result},
        txn::Txn,
    },
    utils::signature::{
        BlockSignature, SignatureContextType, TxnSignature, BLOCK_SIGNATURE_CONTEXT,
        TXN_SIGNATURE_CONTEXT,
    },
};

#[derive(Debug)]
pub struct Wallet {
    keypair: ed25519_dalek::Keypair,
}

impl Wallet {
    /// Create a new wallet instance
    ///
    /// Load keypair from file and return wallet instance
    pub fn new_from_file(filepath: &String) -> Self {
        if !filepath.contains("_ed25519") {
            panic!("Filename must have _ed25519 in it: {}", filepath);
        };

        // load the keypair
        let f = File::open(filepath).unwrap();
        let reader = BufReader::new(f);
        let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();

        // open with ed 25519 lib
        let kp = ed25519_dalek::Keypair::from_bytes(&key_json).unwrap();

        Self { keypair: kp }
    }
    /// Create a new wallet instance
    ///
    /// Load keypair and return wallet instance
    pub fn new_from_kp(keypair: ed25519_dalek::Keypair) -> Self {
        Self { keypair }
    }
    fn sign_msg(
        &self,
        prehashed_msg: Sha512,
        sig_ctx: &SignatureContextType,
    ) -> Result<ed25519::Signature> {
        let msg_signature = self.keypair.sign_prehashed(prehashed_msg, Some(sig_ctx))?;

        Ok(msg_signature)
    }
    /// Return the signature for a given txn id/hash.
    ///
    /// Take in id/hash digest, sign digest with current wallet's key, return signature.
    pub fn sign_txn(&self, txn: &Txn) -> TxnSignature {
        let txn_prehash = txn.calc_id_sha512();
        let txn_sig = self.sign_msg(txn_prehash, TXN_SIGNATURE_CONTEXT).unwrap();

        txn_sig.into()
    }

    pub fn sign_block(&self, block: &Block) -> BlockSignature {
        let block_prehash = block.calc_id_sha512();
        let block_sig = self
            .sign_msg(block_prehash, BLOCK_SIGNATURE_CONTEXT)
            .unwrap();

        block_sig.into()
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// Get the public key for this respective wallet
    pub fn pbkey(&self) -> PbKey {
        self.keypair.public
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////

    /// Create JSON file containing keypair as a u8-byte array
    ///
    /// if you dont have a key, create one.
    ///
    /// File must be `.json`.
    pub fn create_random_key(filepath: String) -> Result<()> {
        if !filepath.contains(".json") {
            return Err(format_err!("Please specify json filetype."));
        }
        let secp = secp256k1::Secp256k1::new();
        let mut rng = rngs::StdRng::seed_from_u64(9);
        let (priv_key, _pub_key) = secp.generate_keypair(&mut rng);

        let secret_bytes = priv_key.secret_bytes();
        let data = secret_bytes.to_vec();

        let f = File::create(filepath).unwrap();
        let mut writer = BufWriter::new(f);
        serde_json::to_writer(&mut writer, &data).unwrap();
        writer.flush().unwrap();

        Ok(())
    }

    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////

    /// ## Validate a message.
    /// Intended for ed25519
    pub fn validate_msg(&self, msg: &Vec<u8>, signature: &Vec<u8>) -> Result<()> {
        let signature = ed25519_dalek::Signature::from_bytes(&signature).unwrap();

        Ok(self.keypair.verify(msg, &signature)?)
    }

    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}
