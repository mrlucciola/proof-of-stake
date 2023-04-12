// imports
use chrono::prelude::*;
use ed25519_dalek::{Digest, Sha512};
use serde::Serialize;
use serde_big_array::BigArray;
// local
use crate::{
    ledger::{
        blockchain::BlockMapKey, general::PbKey, general::Result, txn::Txn, txn_pool::TxnMap,
        wallet::Wallet,
    },
    utils::signature::{BlockSignature, BLOCK_SIGNATURE_CONTEXT},
};

// export types
#[derive(Debug, Serialize, Clone, Copy)]
pub struct BlockId(#[serde(with = "BigArray")] pub [u8; 64]);
impl From<Sha512> for BlockId {
    fn from(value: Sha512) -> Self {
        // let mut test = Sha512;
        let mut test: [u8; 64] = value.finalize().into();
        BlockId(test)
    }
}
pub type BlockIdSha512 = Sha512;

/// This is TxnMap with added functionality.
///
/// @todo add condition that this map cant have more than _ number of txns.
pub type BlockTxnMap = TxnMap;

pub const BLOCK_MSG_CTX: &[u8; 8] = b"block-v0";

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
    pub id: Option<BlockIdSha512>,
    /// the leader's signature for this block submission - Ecdsa signature
    #[serde(skip_serializing)]
    pub signature: Option<BlockSignature>,
}

impl Block {
    /// `Block` constructor fxn - create a new unsigned block (not genesis block).
    ///
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
    /// Convert to bytes - NOT id/hash/message/digest
    /// TODO: replace `Vec<u8>` - don't allocate if possible
    pub fn to_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing block")
    }
    /// ## Calculate the id (blockhash) for a `Block`.
    /// Converts semantic data for the block - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> Sha512 {
        // Create a hash digest object which we'll feed the message into:
        let mut prehashed: Sha512 = Sha512::new();

        // add the block version
        prehashed.update(BLOCK_MSG_CTX);
        // add the block bytes
        prehashed.update(self.to_bytes());
        // return the hash digest - the block's id
        prehashed
    }

    /// Create and return a block signature based on
    ///    the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.sign_block(self)
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// ### Get `Block.id` property.
    pub fn id(&self) -> BlockId {
        self.id.unwrap().into()
    }
    /// ### Get `Block.id` in type used as a key for `BlockMap`.
    /// @todo change to byte array
    pub fn id_key(&self) -> BlockMapKey {
        self.id().to_hex().to_string()
    }
    /// ### Get `Block.transactions` included in this `Block`.
    pub fn txns(&self) -> &BlockTxnMap {
        &self.txns
    }
    /// ### Get `Block.signature` property.
    /// Can return `None` if not yet signed.
    pub fn signature(&self) -> Option<&BlockSignature> {
        self.signature.as_ref()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// ## Set the signature for the block.
    fn set_signature(&mut self, signature: BlockSignature) {
        self.signature = Some(signature);
    }

    /// ## Add the signature to the block body in place.
    ///
    /// 1) Sign the block hash
    /// 2) Add signature to `Block` body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> BlockSignature {
        let signature = self.calc_signature(wallet);
        self.set_signature(signature.clone());

        signature
    }

    /// ## Calculate and set the id for a `Block`.
    /// Returns id.
    pub fn set_id(&mut self) -> BlockIdSha512 {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }

    /// ## Add a transaction to the block.
    ///
    /// Since we are updating the state of the block, we update the block id (hash) here.
    pub fn add_txn(&mut self, new_txn: Txn) {
        self.txns.entry(new_txn.id_key()).or_insert(new_txn);
        // update block hash since the transactions map has been updated
        self.set_id();
    }

    ////////////////////////////// SETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////// UTILS ///////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ///////////////////////////// VALIDATION ////////////////////////////

    /// ## Check if signature is valid.
    ///
    /// 1. Assert there is a signature
    /// 1. Assert signature is valid
    ///
    /// Return is a Result of an Option to handle non-existant signatures
    ///   - `Some()` indicates a signature exists and its valid/invalid
    ///   - `None` indicates there is no signature
    ///   - `Error` is for error handling
    pub fn is_signature_valid(&self, signer_pbkey: &PbKey) -> Result<()> {
        // 1) check if signature exists
        if let None = self.signature() {
            return Err(BlockError::EmptySignature.into());
        }

        let mut prehashed_message: Sha512 = Sha512::new();
        prehashed_message.update(BLOCK_MSG_CTX);
        prehashed_message.update(self.to_bytes());

        let block_signature = self.signature.clone().unwrap();
        let is_verified = signer_pbkey.verify_prehashed(
            prehashed_message,
            Some(BLOCK_SIGNATURE_CONTEXT),
            &block_signature.clone().into(),
        )?;

        Ok(is_verified)
    }

    /// ## Check if block is valid.
    ///
    /// Valid criteria:
    ///   - all struct properties are not `None`
    ///   - hash is valid
    ///   - signature is valid
    pub fn is_valid(&self, signer_pbkey: &PbKey) -> Result<()> {
        // validate fields
        if let None = self.signature {
            return Err(BlockError::EmptySignature.into());
        }
        if let None = self.id {
            return Err(BlockError::EmptyId.into());
        };

        // validate hash
        // TODO: decide - we can validate the id-hash (fast) or recalculate the hash and compare (slow). Is there a major performance hit?
        if self.calc_id() != self.id() {
            return Err(BlockError::IncorrectId.into());
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
    // EmptySignature(#[repr(C)] anyhow::Error),
    #[error("Invalid block: No signature")]
    EmptySignature,
    #[error("Invalid block: No ID")]
    EmptyId,
    #[error("Incorrect ID")]
    IncorrectId,
}
