// imports
use anyhow::format_err;
use secp256k1::{
    rand::{rngs, SeedableRng},
    KeyPair, Message, Secp256k1,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
// local
use crate::{
    ledger::{
        blocks::BlockId,
        general::{PbKey, Result, SecpError},
        txn::{Txn, TxnId},
    },
    utils::signature::{BlockSignature, TxnSignature},
};

pub struct Wallet {
    keypair: KeyPair,
}

impl Wallet {
    /// Create a new wallet instance
    ///
    /// Load keypair from file and return wallet instance
    pub fn new_from_file(filepath: &String) -> Self {
        // load the keypair
        let f = File::open(filepath).unwrap();
        let reader = BufReader::new(f);
        let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();
        let secp = Secp256k1::new();

        let keypair = KeyPair::from_seckey_slice(&secp, &key_json).unwrap();

        Self { keypair }
    }
    /// Create a new wallet instance
    ///
    /// Load keypair and return wallet instance
    pub fn new_from_kp(keypair: &KeyPair) -> Self {
        Self {
            keypair: keypair.clone(),
        }
    }

    pub fn msg_from_id_txn(txn_id: &TxnId) -> Message {
        secp256k1::Message::from_slice(txn_id.as_bytes()).unwrap()
    }
    pub fn msg_from_id_block(block_id: &BlockId) -> Message {
        secp256k1::Message::from_slice(block_id.as_bytes()).unwrap()
    }
    /// Return the signature for a given txn id/hash.
    ///
    /// Take in id/hash digest, sign digest with current wallet's key, return signature.
    pub fn sign_txn(&self, txn_id: &TxnId) -> TxnSignature {
        // convert to correct message type
        self.sign_id_txn(txn_id)
    }
    pub fn sign_block(&self, block_id: &BlockId) -> BlockSignature {
        self.sign_id_block(block_id)
    }

    pub fn sign_id_txn(&self, txn_id: &TxnId) -> TxnSignature {
        let msg = Self::msg_from_id_txn(txn_id);
        self.sign_msg(msg)
    }
    pub fn sign_id_block(&self, block_id: &BlockId) -> BlockSignature {
        let msg = Self::msg_from_id_block(block_id);
        self.sign_msg(msg)
    }
    pub fn sign_msg(&self, msg: Message) -> TxnSignature {
        let secp = Secp256k1::new();
        secp.sign_ecdsa(&msg, &self.keypair.secret_key())
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// Get the public key for this respective wallet
    pub fn pbkey(&self) -> PbKey {
        self.keypair.public_key()
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

    pub fn validate_txn_signature(txn: &Txn, signature: &TxnSignature, pbkey: &PbKey) -> bool {
        let secp = Secp256k1::new();
        let is_valid =
            // TODO: fix the signature field
            match secp.verify_ecdsa(&Message::from_slice(txn.id().as_bytes()).unwrap(), &signature, pbkey) {
                Ok(_) => true,
                Err(SecpError::IncorrectSignature) => false,
                Err(e) => panic!("Signature validation: {}", e),
            };

        is_valid
    }

    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}
