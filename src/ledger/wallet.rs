use super::txn::Txn;

// TODO: use a library for rsa keypair
pub struct RsaKeypair();
pub struct TxnHash();
pub struct TxnSignature();
pub struct TxnUtils();
impl TxnUtils {
    pub fn hash(_data: Txn) -> TxnHash {
        TxnHash()
    }
    pub fn signature(_txn_hash: TxnHash) -> TxnSignature {
        TxnSignature()
    }

    /// Takes the txn data object and returns a signature
    pub fn get_txn_signature(_data: Txn) -> TxnSignature {
        let txn_hash = Self::hash(_data);

        Self::signature(txn_hash)
    }
}

pub struct Wallet {
    pub keypair: RsaKeypair,
}

impl Wallet {
    /// Create a new wallet instance
    ///
    /// 1. create keypair
    pub fn new() -> Self {
        let keypair = RsaKeypair();

        Self { keypair }
    }

    pub fn sign(&self, data: Txn) {
        let txn_signature = TxnUtils::get_txn_signature(data);
    }
}
