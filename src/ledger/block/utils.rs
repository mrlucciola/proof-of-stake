// external
use ed25519_dalek::Digest;
// local
use crate::ledger::{
    block::{constants::BLOCK_MSG_CTX, types::BlockDigest, Block, BlockId, BlockSignature},
    general::Sha512,
    wallet::Wallet,
};

impl Block {
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
    /// ### Calculate the pre-hash struct for the id.
    /// This is ONLY used with calc-id method.
    ///
    /// @todo add more block information to the block hasher.
    fn calc_id_sha512_prehash(&self) -> Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehash: Sha512 = Sha512::new();
        // add the block version
        prehash.update(BLOCK_MSG_CTX);
        // add the block header bytes
        prehash.update(self.header.serialize());
        // @todo add other info to the prehash
        // return the hasher/prehash struct
        prehash
    }

    /// ### Create and return a block signature based on the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.sign_block(self)
    }
}
