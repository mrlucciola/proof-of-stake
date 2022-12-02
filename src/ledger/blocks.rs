// imports
use chrono::prelude::*;
use serde::Serialize;
use std::collections::BTreeMap;
// local
use crate::{
    ledger::{
        blockchain::BlockMapKey,
        general::PbKey,
        txn::{Txn, TxnMapKey},
        wallet::Wallet,
    },
    utils::blake_hash::{BlakeHash, BlakeHex},
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
        transactions: BlockTxnMap,
        leader: PbKey,
        prev_block_id: BlockId,
        prev_blockheight: u128,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();
        let blockheight = prev_blockheight + 1;

        let mut block = Self {
            transactions,
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
    pub fn as_bytes(&self) -> Vec<u8> {
        // serialize to a byte vector
        serde_json::to_vec(&self).expect("Error serializing block")
    }
    /// Calculate, set and return the id for a `Block`.
    ///
    /// Converts semantic data for the block - all non-calculated fields (i.e. excludes `id` and `signature`) into bytes.
    ///
    /// Hashes this info and produces a digest - the ID.
    pub fn calc_id(&self) -> BlockId {
        let mut hasher = blake3::Hasher::new();
        // version
        hasher.update(b"block-v0");
        // add the block bytes
        hasher.update(&self.as_bytes());
        // return the hash digest - the block's id
        hasher.finalize().into()
    }
    /// Get and set the id for a `Block`.
    ///
    /// Returns id
    pub fn set_id(&mut self) -> BlockId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }
    /// Method wrapper/analog for `self.calc_id()`
    ///
    /// TODO: add `Result`
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
    /// prev: get_signature
    pub fn calc_signature(&self, wallet: &Wallet) -> BlockSignature {
        wallet.get_signature(&self.id().as_bytes())
    }
    /// Set the signature for the block
    fn set_signature(&mut self, signature: Option<BlockSignature>) {
        self.signature = signature;
    }
    /// Add the signature to the block body in place.
    ///
    /// 1) Sign the block hash
    /// 2) Add signature to `Block` body
    /// 3) Return signature
    pub fn sign(&mut self, wallet: &Wallet) -> BlockSignature {
        let sig: BlockSignature = self.calc_signature(wallet);
        self.set_signature(Some(sig));

        sig
    }
}
