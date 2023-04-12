use ed25519_dalek::Digest;

use crate::{
    ledger::{general::Sha512, wallet::Wallet},
    utils::signature::TxnSignature,
};

use super::{Txn, TxnDigest, TxnId, TXN_MSG_CTX};

impl Txn {
    /// ## Convert transaction struct to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing txn")
    }

    /// ## Compute the id (hash digest) of the transaction.
    /// Converts semantic data for the txn - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> TxnId {
        let prehash = self.calc_id_sha512_prehash();

        // return the hash digest - the block's id
        let digest: TxnDigest = prehash.finalize().into();
        TxnId(digest)
    }
    /// ## Calculate the pre-hash struct for the id
    pub fn calc_id_sha512_prehash(&self) -> Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehash: Sha512 = Sha512::new();
        // add the txn version
        prehash.update(TXN_MSG_CTX);
        // add the txn bytes
        prehash.update(self.to_bytes());
        // return the hasher/prehash struct
        prehash
    }

    /// ## Create and return a message signature based on the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> TxnSignature {
        wallet.sign_txn(self)
    }
}
