// imports
use anyhow::format_err;
use secp256k1::{
    rand::{rngs, SeedableRng},
    Error as SecpError, KeyPair, Message, Secp256k1,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
// local
use crate::{
    ledger::{
        blocks::BlockId,
        general::{PbKey, Result},
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
    /// Return the signature for a given txn id/hash.
    ///
    /// Take in id/hash digest, sign digest with current wallet's key, return signature.
    pub fn sign_txn(&self, txn_id: &TxnId) -> TxnSignature {
        // convert to correct message type
        TxnSignature::sign_id(txn_id, &self.keypair.secret_key())
    }
    pub fn sign_block(&self, block_id: &BlockId) -> BlockSignature {
        BlockSignature::sign_id(block_id, &self.keypair.secret_key())
    }

    pub fn validate_txn_signature(txn: &Txn, signature: &TxnSignature, pbkey: &PbKey) -> bool {
        let secp = Secp256k1::new();
        let is_valid =
            // TODO: fix the signature field
            match secp.verify_ecdsa(&Message::from_slice(txn.id().as_bytes()).unwrap(), &signature.0.0, pbkey) {
                Ok(_) => true,
                Err(SecpError::IncorrectSignature) => false,
                Err(e) => panic!("Signature validation: {}", e),
            };

        is_valid
    }

    /// Get the public key for this respective wallet
    pub fn pbkey(&self) -> PbKey {
        self.keypair.public_key()
    }

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
}
