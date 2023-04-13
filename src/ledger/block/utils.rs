// external
use ed25519_dalek::Digest;
// local
use super::{constants::BLOCK_MSG_CTX, types::BlockDigest, Block, BlockId, BlockSignature};
use crate::ledger::{general::Sha512, wallet::Wallet};

impl Block {
    /// ### Convert to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate if possible
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing block")
    }
    /// ### Calculate the id (blockhash) for a `Block`.
    /// Converts semantic data for the block - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a hash digest - the ID.
    pub fn calc_id(&self) -> BlockId {
        let prehash = self.calc_id_sha512_prehash();

        // return the hash digest - the block's id
        let digest: BlockDigest = prehash.finalize().into();

        BlockId(digest)
    }
    /// ### Calculate the pre-hash struct for the id
    pub fn calc_id_sha512_prehash(&self) -> Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehash: Sha512 = Sha512::new();
        // add the block version
        prehash.update(BLOCK_MSG_CTX);
        // add the block bytes
        prehash.update(self.to_bytes());
        // return the hasher/prehash struct
        prehash
    }

    /// ### Create and return a block signature based on the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.sign_block(self)
    }
}
