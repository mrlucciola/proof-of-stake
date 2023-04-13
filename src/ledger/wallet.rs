// imports
use {
    anyhow::format_err,
    ed25519_dalek::Signer,
    secp256k1::rand::{rngs, SeedableRng},
    std::{
        fs::File,
        io::{BufReader, BufWriter, Write},
    },
};
// local
use crate::ledger::{
    block::{constants::*, Block, BlockSignature},
    general::{PbKey, Result, KP},
    txn::{constants::*, Txn, TxnSignature},
};

#[derive(Debug)]
pub struct Wallet {
    keypair: KP,
}

impl Wallet {
    /// ## Create a new wallet instance
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
        let kp = KP::from_bytes(&key_json).unwrap();

        Self { keypair: kp }
    }
    /// ## Create a new wallet instance
    /// Load keypair and return wallet instance
    pub fn new_from_kp(keypair: KP) -> Self {
        Self { keypair }
    }
    /// ## Return the signature for a given txn id/hash.
    /// Take in id/hash digest, sign digest with current wallet's key, return signature.
    pub fn sign_txn(&self, txn: &Txn) -> TxnSignature {
        self.sign_msg(&mut txn.calc_id().0, TXN_SIGNATURE_CTX)
            .into()
    }

    /// ## Sign a block. Generate signature for block.
    /// We are not using the prehash Sha512 for consistency, modularity and ease of use.\
    /// Also, there may be significant or breaking changes in the future as suggested in their documentation (r.e. "bandaids").
    pub fn sign_block(&self, block: &Block) -> BlockSignature {
        self.sign_msg(&mut block.calc_id().0, BLOCK_SIGNATURE_CONTEXT)
            .into()
    }
    /// ## Standard function for signing messages.
    /// It is important to enforce consistency in how msgs are signed.
    fn sign_msg(&self, msg: &mut [u8; 64], ctx: &[u8]) -> ed25519::Signature {
        let mut vector = ctx.to_vec();
        vector.append(&mut msg.to_vec());

        // sign msg and return signature
        self.keypair.sign(&vector)
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// ### Get the public key for this respective wallet
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
