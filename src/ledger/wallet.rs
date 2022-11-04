// imports
use anyhow::{format_err, Result};
use secp256k1::{
    ecdsa::Signature as TxnSignature,
    rand::{rngs, SeedableRng},
    Error as SecpError, KeyPair, Message, PublicKey, Secp256k1,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
// local
use super::txn::Txn;

pub struct Wallet {
    /// TODO: make private
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

    pub fn get_msg_signature(&self, txn_msg: &Message) -> TxnSignature {
        let secp = Secp256k1::new();
        let sig: TxnSignature = secp.sign_ecdsa(&txn_msg, &self.keypair.secret_key());

        sig
    }

    /// Add the signature to the transaction body.
    ///
    /// 1) Sign the transaction
    /// 2) Return signature
    pub fn sign(&self, txn: &mut Txn) -> TxnSignature {
        // get the txn message
        let msg = txn.get_txn_msg();
        // sign the txn
        let sig = self.get_msg_signature(&msg);

        txn.signature = Some(sig);

        sig
    }

    pub fn validate_signature(txn: &Txn, signature: &TxnSignature, pbkey: &PublicKey) -> bool {
        // let txn_hash = txn_data.hash();
        let secp = Secp256k1::new();
        let is_valid = match secp.verify_ecdsa(&txn.get_txn_msg(), signature, pbkey) {
            Ok(_) => true,
            Err(SecpError::IncorrectSignature) => false,
            Err(e) => panic!("Signature validation: {}", e),
        };

        is_valid
    }

    /// Get the public key for this respective wallet
    pub fn get_pbkey(&self) -> PublicKey {
        self.keypair.public_key()
    }

    /// if you dont have a key, create one
    /// filepath must be json
    pub fn create_key(filepath: String) -> Result<()> {
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
