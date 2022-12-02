use arrayvec::ArrayString;
use base64::display::Base64Display;
// imports
// use blake3::Hash as BlakeHash;
use chrono::prelude::*;
use serde::{Serialize, Serializer};
use std::{collections::BTreeMap, hash::Hash};
// local
use crate::ledger::{
    blockchain::BlockMapKey,
    general::PbKey,
    txn::{Txn, TxnMapKey},
    wallet::Wallet,
};

// export types
pub type BlockId = BlakeHash; // TODO: change to hex
pub type BlockSignature = [u8; 64];

// TODO: add condition that this map cant have more than _ number of txns
pub type BlockTxnMap = BTreeMap<TxnMapKey, Txn>;

/// Info contained within a block
#[derive(Debug, Clone, Serialize)]
pub struct Block {
    /// list/map of all transactions to be included in the block
    transactions: BlockTxnMap,
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
    pub id: BlockId,
    /// the leader's signature for this block submission - Ecdsa signature
    #[serde(skip_serializing)]
    pub signature: BlockSignature,
}

impl Block {
    /// `Block` constructor fxn - create a new block (not genesis block).
    ///
    /// transactions: List of transactions (`Txn`) to be included in the block\
    /// TODO: add `blockchain` as param - use it to get block count
    pub fn new(
        transactions: BlockTxnMap,
        leader: PbKey,
        prev_block_id: BlockId,
        prev_blockheight: u128,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();
        let blockheight = prev_blockheight + 1;
        let init_id = BlakeHash::from_bytes([0u8; 32]);
        let init_sig = [0u8; 64];

        let mut block = Self {
            transactions,
            leader,
            prev_block_id,
            blockheight,
            system_time,
            id: init_id,
            signature: init_sig,
        };

        // set the id (hash) with the body
        block.set_id();
        block
    }
    pub fn digest(&self) {
        // serialize to a byte vector
        let block_digest: Vec<u8> = serde_json::to_vec(&self).expect("Error serializing block");
    }
    pub fn calc_id(&self) -> BlakeHash {
        // get hash digest of block
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"block-v0");
        hasher.update(&block_digest);
        let id_abstract: BlakeHash = hasher.finalize();

        id_abstract
    }
    /// Get and set the id for a `Block`.
    ///
    /// Returns id
    pub fn set_id(&mut self) -> BlockId {
        let id = Self::calc_id(self);
        self.id = id;

        id
    }
    /// Method wrapper/analog for `self::calc_id()`
    pub fn id(&self) -> BlockId {
        Self::calc_id(&self).as_bytes().to_owned()
    }
    /// Get Block Id in `String` form
    /// TODO: incorrect id type
    pub fn id_str(&self) -> String {
        // let id = self.id();
        // String::from_utf8_lossy(&id.to_vec()).to_string()
        Self::calc_id(self).to_string()
    }
    /// Get Block Id in `hex` form.
    ///
    /// TODO: convert `Self::calc_id()` to `self.id()`
    pub fn id_hex(&self) -> String {
        let id = Self::calc_id(self);
        id.to_string()
    }
    /// Get `BlockMap` key type (derived from BlockId)
    ///
    /// The Block Map key type is currently `String` (could be hex string)
    pub fn id_key(&self) -> BlockMapKey {
        self.id_str()
    }
    /// Compute the ID (hash digest) of the block - associated fxn
    ///
    /// Zero-out the `id` and `signature` in order to compute properly
    ///
    /// TODO: abstract out the `block`-specific attributes to separate `block` struct.
    ///     Create new `UnsignedBlock` struct - `{ block: Block, id: BlockId }`
    ///     Create new `SignedBlock` struct - `{ block: Block, id: BlockId, signature: BlockSignature }`
    /// TODO: change BlakeHash to BlockId
    pub fn _calc_id(block: &Block) -> BlakeHash {
        let mut adj_block_body = block.clone();
        // set blank vars
        adj_block_body.id = [0u8; 32];
        adj_block_body.signature = [0u8; 64];

        // serialize to a byte vector
        let block_msg_bytes: Vec<u8> = match serde_json::to_vec(&adj_block_body) {
            Ok(b) => b,
            Err(err) => {
                panic!("\n\nHelp - block_msg_bytes: \n{:?}\n\n", err)
            }
        };

        // get hash digest of block
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"block-v0");
        hasher.update(&block_msg_bytes);
        let id_abstract: BlakeHash = hasher.finalize();

        id_abstract
    }
    /// Add a transaction to the block.
    ///
    /// Since we are updating the state of the block, we update the block id (hash) here.
    pub fn add_txn(&mut self, new_txn: Txn) {
        // TODO: change to Txn.key()
        self.transactions
            // .entry(new_txn.key())
            .entry(new_txn.id_str())
            .or_insert(new_txn);
        // update block hash since the transactions map has been updated
        self.set_id();
    }
    /// Getter for `transactions` mapping.
    ///
    /// There should be no direct access to the `transactions` mapping.
    pub fn transactions(&self) -> &BlockTxnMap {
        &self.transactions
    }
    /// Create and return a block signature based on
    ///    the contents of the transaction
    pub fn get_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.get_signature(&self.id())
    }
    /// Set the signature for the block
    pub fn set_signature(&mut self, signature: BlockSignature) {
        self.signature = signature;
    }
    /// Add the signature to the block body in place.
    ///
    /// 1) Sign the block hash
    /// 2) Add signature to `Block` body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> BlockSignature {
        let sig: BlockSignature = self.get_signature(wallet);
        self.set_signature(sig);
        sig
    }
}
