// imports
use ed25519_dalek::Digest;
// local
use crate::ledger::{
    general::Sha512,
    txn::{Txn, TxnDigest, TxnId, TxnSignature, TXN_MSG_CTX},
    wallet::Wallet,
};

impl Txn {
    /// ### Compute the id (hash digest) of the transaction.
    /// Converts semantic data for the txn - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> TxnId {
        let prehash = self.calc_id_sha512_prehash();

        // return the hash digest - the block's id
        let digest: TxnDigest = prehash.finalize().into();

        TxnId(digest)
    }
    /// ### Calculate the pre-hash struct for the id.
    /// This is only used in the `calc_id` method.
    ///
    /// This method is abstracted out to allow calc-id be adapted for
    /// different hashing algorithms.
    fn calc_id_sha512_prehash(&self) -> Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehash: Sha512 = Sha512::new();
        // add the txn version
        prehash.update(TXN_MSG_CTX);
        // add the txn bytes
        prehash.update(self.header.serialize());
        // return the hasher/prehash struct
        prehash
    }

    /// ### Create and return a message signature based on the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> TxnSignature {
        wallet.sign_txn(self)
    }

    /// ### Add the signature to the transaction body in place.
    /// 1) Create the signature - sign the transaction;
    /// 2) Add signature to transaction body;
    /// 3) Return signature;
    pub fn sign(&mut self, wallet: &Wallet) -> TxnSignature {
        let sig = self.calc_signature(wallet);
        self.set_signature(sig.to_owned());

        sig
    }
}
