mod block_id;
pub mod constants;
mod getters;
mod setters;
// imports
use {chrono::prelude::*, ed25519_dalek::Digest, serde::Serialize, serde_big_array::BigArray};
// local
use crate::{
    ledger::{
        general::{PbKey, Sha512},
        txn::Txn,
        txn_pool::TxnMap,
        wallet::Wallet,
    },
    utils::signature::BlockSignature,
};
pub use {block_id::BlockId, constants::*};

/// ### This is TxnMap with added functionality.
/// @todo add condition that this map cant have more than _ number of txns.
pub type BlockTxnMap = TxnMap;

/// Info contained within a block
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    /// list/map of all transactions to be included in the block
    txns: BlockTxnMap,
    /// public key of the current block proposer (node)
    pub leader: [u8; 32],
    /// Identifier of the previous block - hash digest
    pub prev_block_id: BlockId,
    /// block height - current number of blocks in blockchain + 1
    pub blockheight: u128,
    /// current time - unix time stamp
    pub system_time: u64,
    /// Identifier/ID - hash digest of the current block
    #[serde(skip_serializing)]
    pub id: Option<BlockId>,
    /// the leader's signature for this block submission - Ecdsa signature
    #[serde(skip_serializing)]
    pub signature: Option<BlockSignature>,
}

impl Block {
    /// ## `Block` constructor fxn - create a new unsigned block (not genesis block).
    /// transactions: List of transactions (`Txn`) to be included in the block\
    /// TODO: add `blockchain` as param - use it to get block count
    /// @todo allow `None` input for `txns` to default to a new block txn map
    pub fn new(
        txns: BlockTxnMap,
        leader: PbKey,
        prev_block_id: BlockId,
        prev_blockheight: u128,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();
        let blockheight = prev_blockheight + 1;

        let mut block = Self {
            txns,
            leader: leader.to_bytes(),
            prev_block_id,
            blockheight,
            system_time,
            id: None,
            signature: None,
        };

        // set the id (hash) with the body
        block.set_id();
        block
    }
    /// ## Convert to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate if possible
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing block")
    }
    /// ## Calculate the id (blockhash) for a `Block`.
    /// Converts semantic data for the block - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a hash digest - the ID.
    pub fn calc_id(&self) -> BlockId {
        let prehash = self.calc_id_sha512_prehash();

        // return the hash digest - the block's id
        let digest: [u8; 64] = prehash.finalize().into();
        BlockId(digest)
    }
    /// ## Calculate the pre-hash struct for the id
    pub fn calc_id_sha512_prehash(&self) -> Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehashed: Sha512 = Sha512::new();
        // add the block version
        prehashed.update(BLOCK_MSG_CTX);
        // add the block bytes
        prehashed.update(self.to_bytes());
        // return the hasher/prehash struct
        prehashed
    }

    /// ## Create and return a block signature based on the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.sign_block(self)
    }

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////

    /// ### Check if signature is valid.
    ///
    /// 1. Assert there is a signature
    /// 1. Assert signature is valid
    ///
    /// Return is a Result of an Option to handle non-existant signatures
    /// - `Some()` indicates a signature exists and its valid/invalid
    /// - `None` indicates there is no signature
    /// - `Error` is for error handling
    pub fn is_signature_valid(&self, signer_pbkey: &PbKey) -> std::result::Result<(), BlockError> {
        // 1) check if signature exists
        if let None = self.signature() {
            return Err(BlockError::EmptySignature.into());
        }

        // create message for verification
        let msg: [u8; 64] = self.calc_id().into();
        let mut presigned_msg = BLOCK_SIGNATURE_CONTEXT.to_vec();
        presigned_msg.append(&mut msg.to_vec());

        // get the current signature
        let block_signature = self.signature.clone().unwrap();
        let sig_test = ed25519::Signature::from_bytes(&block_signature.0).unwrap();

        match signer_pbkey.verify_strict(&presigned_msg, &sig_test) {
            Ok(_) => Ok(()),
            Err(e) => Err(BlockError::InvalidSignature(e, sig_test)),
        }
    }

    /// ### Check if block is valid.
    ///
    /// Valid criteria:
    ///   - all struct properties are not `None`
    ///   - hash is valid
    ///   - signature is valid
    pub fn is_valid(&self, signer_pbkey: &PbKey) -> std::result::Result<(), BlockError> {
        // validate fields
        if let None = self.signature {
            return Err(BlockError::EmptySignature);
        }
        if let None = self.id {
            return Err(BlockError::EmptyId);
        };

        // validate hash
        // TODO: decide - we can validate the id-hash (fast) or recalculate the hash and compare (slow). Is there a major performance hit?
        if self.calc_id() != self.id() {
            return Err(BlockError::IncorrectId);
        }

        // validate signature
        self.is_signature_valid(&signer_pbkey)?;

        Ok(())
    }

    ///////////////////////////// VALIDATION ////////////////////////////
    /////////////////////////////////////////////////////////////////////
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockError {
    #[error("BlockError::EmptySignature- Invalid block: No signature")]
    EmptySignature,
    #[error("BlockError::EmptyId- Invalid block: No ID")]
    EmptyId,
    #[error("BlockError::IncorrectId- Incorrect ID")]
    IncorrectId,
    #[error("BlockError::InvalidSignature- {0}. Testing signature:\n{1}")]
    InvalidSignature(ed25519_dalek::SignatureError, ed25519::Signature),
}
