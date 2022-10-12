use anyhow::{format_err, Result};
// imports
// use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use secp256k1::{
    ecdsa::Signature as EcdsaSignature,
    rand::{rngs, SeedableRng},
    KeyPair, Message, Secp256k1,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};
// local
use super::txn::Txn;

pub struct TxnHash();
pub struct TxnSignature();
// pub struct TxnUtils();
// impl TxnUtils {
//     pub fn signature(_txn_hash: TxnHash) -> TxnSignature {
//         TxnSignature()
//     }

//     /// Takes the txn data object and returns a signature
//     pub fn get_txn_signature(_data: Txn) -> TxnSignature {
//         let txn_hash = Txn::hash_txn(_data);

//         Self::signature(txn_hash)
//     }
// }

pub struct Wallet {
    /// TODO: make private
    keypair: KeyPair,
}

impl Wallet {
    /// Create a new wallet instance
    ///
    /// 1. load keypair
    /// TODO: input from file
    pub fn new(filepath: &String) -> Self {
        // load the keypair
        let f = File::open(filepath).unwrap();
        let reader = BufReader::new(f);
        let key_json: Vec<u8> = serde_json::from_reader(reader).unwrap();
        let secp = Secp256k1::new();

        let keypair = KeyPair::from_seckey_slice(&secp, &key_json).unwrap();

        Self { keypair }
    }

    /// Add the signature to the transaction body.
    ///
    /// 1) Sign the transaction
    /// 2) Return signature
    pub fn sign(&self, txn_data: &Txn) -> EcdsaSignature {
        let _txn_hash = txn_data.hash();

        let secp = Secp256k1::new();

        // TODO: get correct message body to hash
        let msg_bytes = &[0xab; 32];
        let msg = Message::from_slice(msg_bytes).expect("trying to get txn message");

        // 1) sign
        let sig = secp.sign_ecdsa(&msg, &self.keypair.secret_key());

        // 2) return transaction signature
        sig
    }

    // Signature is a ed25519 signature
    pub fn signature_valid(
        wallet: &Self,
        txn_data: Txn,
        signature: EcdsaSignature,
        _pub_key_str: &String,
    ) -> bool {
        let pub_key = wallet.keypair.public_key();

        let secp = Secp256k1::verification_only();

        // TODO: get correct message body to hash
        let _msg_hash = txn_data.hash();
        let msg_bytes = &[0xab; 32];
        let msg = Message::from_slice(msg_bytes).expect("trying to get txn message");

        // return bool
        secp.verify_ecdsa(&msg, &signature, &pub_key).is_ok()
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
