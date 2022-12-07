// imports
use chrono::prelude::*;
use secp256k1::Secp256k1;
use serde::Serialize;
use std::collections::BTreeMap;
// local
use crate::{
    ledger::{
        blockchain::BlockMapKey,
        general::{PbKey, SecpError},
        txn::Txn,
        txn_pool::TxnMapKey,
        wallet::Wallet,
    },
    utils::{
        hash::{BlakeHash, BlakeHex},
        signature::BlockSignature,
    },
};

use super::general::Result;

// export types
pub type BlockId = BlakeHash;

// TODO: add condition that this map cant have more than _ number of txns
pub type BlockTxnMap = BTreeMap<TxnMapKey, Txn>;

/// Info contained within a block
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    /// list/map of all transactions to be included in the block
    txns: BlockTxnMap,
    /// public key of the current block proposer (node)
    pub leader: PbKey,
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
    /// `Block` constructor fxn - create a new block (not genesis block).
    ///
    /// transactions: List of transactions (`Txn`) to be included in the block\
    /// TODO: add `blockchain` as param - use it to get block count
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
            leader,
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
    pub fn as_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing block")
    }
    /// Calculate the id (blockhash) for a `Block`.
    ///
    /// Converts semantic data for the block - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> BlockId {
        let mut hasher = blake3::Hasher::new();
        // add the block version
        hasher.update(b"block-v0");
        // add the block bytes
        hasher.update(&self.as_bytes());
        // return the hash digest - the block's id
        hasher.finalize().into()
    }

    /// Create and return a block signature based on
    ///    the contents of the transaction
    pub fn calc_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.sign_block(&self.id())
    }

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// GETTERS //////////////////////////////

    /// Getter for `Block` `id` property.
    pub fn id(&self) -> BlockId {
        self.id.unwrap()
    }
    /// Get Block Id in `String` form.
    pub fn id_str(&self) -> String {
        self.id().to_string()
    }
    /// Get Block Id in `hex` form.
    pub fn id_hex(&self) -> BlakeHex {
        self.id().to_hex()
    }
    /// Get Block Id Hex in `String` form.
    pub fn id_hex_string(&self) -> String {
        self.id().to_hex().to_string()
    }
    /// Get `BlockMap` key type (derived from BlockId)
    pub fn id_key(&self) -> BlockMapKey {
        self.id_hex_string()
    }
    /// Getter for `transactions` mapping.
    ///
    /// There should be no direct access to the `transactions` map.
    pub fn txns(&self) -> &BlockTxnMap {
        &self.txns
    }
    /// Getter for `signature` property.
    pub fn signature(&self) -> Option<&BlockSignature> {
        self.signature.as_ref()
    }

    ////////////////////////////// GETTERS //////////////////////////////
    /////////////////////////////////////////////////////////////////////

    /////////////////////////////////////////////////////////////////////
    ////////////////////////////// SETTERS //////////////////////////////

    /// Set the signature for the block
    fn set_signature(&mut self, signature: BlockSignature) {
        self.signature = Some(signature);
    }

    /// Add the signature to the block body in place.
    ///
    /// 1) Sign the block hash
    /// 2) Add signature to `Block` body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> BlockSignature {
        let signature = self.calc_signature(wallet);
        self.set_signature(signature.clone());

        signature
    }

    /// Calculate and set the id for a `Block`.
    ///
    /// Returns id
    /// set_id() -> BlockId
    ///     calc_id() -> BlockId
    ///         blake3::Hasher::new()
    ///         Hasher.finalize()
    pub fn set_id(&mut self) -> BlockId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }

    /// Add a transaction to the block.
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

    /// Check if signature is valid.
    ///
    /// 1. Assert there is a signature
    /// 1. Assert signature is valid
    ///
    /// Return is a Result of an Option to handle non-existant signatures
    ///   - `Some()` indicates a signature exists and its valid/invalid
    ///   - `None` indicates there is no signature
    ///   - `Error` is for error handling
    pub fn is_signature_valid(&self, wallet: &Wallet) -> Result<Option<bool>> {
        // init
        let secp = Secp256k1::new();

        // 1) check if signature exists
        if let None = self.signature() {
            return Ok(None);
        }

        // 2) check if signature is valid
        match secp.verify_ecdsa(
            &secp256k1::Message::from_slice(self.id().as_bytes()).unwrap(),
            self.signature().unwrap(),
            &wallet.pbkey(),
        ) {
            Ok(_) => Ok(Some(true)),
            Err(SecpError::IncorrectSignature) => Ok(Some(false)),
            Err(e) => Err(e.into()),
        }
    }

    /// Check if block is valid.
    ///
    /// Valid criteria:
    ///   - all struct properties are not `None`
    ///   - hash is valid
    ///   - signature is valid
    pub fn is_valid(&self, wallet: &Wallet) -> Result<bool> {
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
        self.is_signature_valid(wallet)?;

        Ok(true)
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
