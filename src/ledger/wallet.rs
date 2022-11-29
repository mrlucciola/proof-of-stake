// imports
use anyhow::{format_err, Result};
use secp256k1::{
    rand::{rngs, SeedableRng},
    Error as SecpError, KeyPair, Message, Secp256k1,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
// local
use crate::ledger::{
    general::{PbKey, SecpEcdsaSignature},
    txn::{Txn, TxnHash, TxnSig},
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
    /// Sign transaction hash with `secp256k1` library, and return the signature.
    ///
    /// For use in `secp256k1` transaction signing.
    fn secp_get_sig_from_txn_hash(&self, txn_hash: &TxnHash) -> TxnSig {
        // Convert byte array to `secp256k1::Message` format
        let msg = secp256k1::Message::from_slice(txn_hash).unwrap();
        // init secp
        let secp = Secp256k1::new();

        let sig = secp.sign_ecdsa(&msg, &self.keypair.secret_key());

        sig.serialize_compact()
    }
    /// Convert compact signature byte array to `Signature` struct.
    ///
    /// For use in `secp256k1` transaction signing.
    pub fn secp_sig_bytes_to_sig_obj(sig_bytes: &TxnSig) -> secp256k1::ecdsa::Signature {
        secp256k1::ecdsa::Signature::from_compact(sig_bytes).unwrap()
    }
    /// Return the signature for a given txn hash/message.
    ///
    /// Take in message/hash digest, sign digest with current wallet's key, return signature.
    pub fn get_signature(&self, txn_hash: &TxnHash) -> TxnSig {
        // convert to correct message type
        self.secp_get_sig_from_txn_hash(txn_hash)
    }

    pub fn validate_signature(txn: &Txn, signature: &SecpEcdsaSignature, pbkey: &PbKey) -> bool {
        let secp = Secp256k1::new();
        let is_valid =
            match secp.verify_ecdsa(&Message::from_slice(&txn.hash()).unwrap(), signature, pbkey) {
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
