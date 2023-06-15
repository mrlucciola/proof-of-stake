pub mod constants;
mod getters;
mod setters;
pub mod txn_id;
pub mod txn_signature;
pub mod types;
mod utils;
// external
use {
    chrono::prelude::*,
    serde::{Deserialize, Serialize},
    std::fmt,
};
// local
use crate::ledger::{general::PbKey, wallet::Wallet};
use constants::*;
pub use {txn_id::TxnId, txn_signature::TxnSignature, types::*};

// exported types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TxnType {
    Transfer = 1,
}
impl fmt::Display for TxnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// ## Transfer transaction.
///
/// This struct represents all information pertaining to a single transfer transaction\
/// along with methods to manipulate it for several usecases throughout the repo.\
/// Transactions are stored in `Block`s and `TxnPool`s.
///
/// ### Flow for creating a transaction:
/// 1. Transaction is instantiatiated by creator `Node` with `Txn.new()`;
/// 1. `Node` (transaction creator, using its keypair via `Wallet`) signs transaction;
/// 1. Transaction is submitted to the `TxnPool`;
/// 1. `Leader` (`Node` chosen to append a `Block` to the `Blockchain`) pulls\
///     transaction (along with 0 or more others) from `TxnPool`;
///     - @todo in future version after mempool is removed:\
///       `Forwarder` (`Node` that has received a transaction) submits transaction to `Leader`;
/// 1. `Leader` `Node` signs block;
///
/// @todo make all fields private, making all accessible fields available thru getter methods.;\
/// @todo generalize this and abstract all separate types.;\
/// @todo create lookup enum for `gas_value` for each `Txn` type;\
/// @todo create `TxnHeader` struct to hold all fields except `id` and `signature`;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Txn {
    pub amt: u128,
    pbkey_send: PbKey,
    pbkey_recv: PbKey,
    // The time the txn was created
    pub system_time: u64,
    /// Type of transaction - as int
    pub txn_type: TxnType,
    /// Transaction identifier: Blake3 hash (currently as byte array)
    id: Option<TxnId>,
    /// Ecdsa signature as byte array
    signature: Option<TxnSignature>,
}

impl Txn {
    /// ## Transaction constructor fxn
    /// Creates a transaction `object`.
    pub fn new(
        pbkey_send: PbKey,
        pbkey_recv: PbKey,
        // amt to send
        amt: u128,
        // type of transaction
        txn_type: TxnType,
    ) -> Self {
        // get the current system time
        let system_time: u64 = Utc::now().timestamp_millis().try_into().unwrap();

        let mut txn = Self {
            pbkey_send,
            pbkey_recv,
            amt,
            system_time,
            txn_type,
            id: None,        //[0u8; 64],
            signature: None, //[0u8; 64],
        };

        // set the id with the body
        txn.set_id();

        // return the txn
        txn
    }

    /// ## Create and return a new signed transaction.
    /// Receives `Wallet` instance for signing.
    ///
    /// Uses `Txn::new()` assoc fxn. to construct the txn, and signs the txn with given wallet.
    pub fn new_signed(
        wallet: &Wallet,
        pbkey_recv: PbKey,
        amt_to_send: u128,
        txn_type: TxnType,
    ) -> Txn {
        let pbkey_send = wallet.pbkey();
        let mut txn = Self::new(pbkey_send, pbkey_recv, amt_to_send, txn_type);

        // add signature to body
        txn.sign(&wallet);

        txn
    }

    /////////////////////////////////////////////////
    //////////////// PRIVATE SETTERS ////////////////
    /// ## Get identifier (hash) for txn and set on txn object and store the output on the Txn object
    /// Returns id.
    fn set_id(&mut self) -> TxnId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }
    /// ## Set the signature for the transaction.
    fn set_signature(&mut self, signature: TxnSignature) {
        self.signature = Some(signature);
    }
    //////////////// PRIVATE SETTERS ////////////////
    /////////////////////////////////////////////////
}
