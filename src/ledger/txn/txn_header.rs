// external
use serde::{Deserialize, Serialize};
// local
use crate::ledger::{general::PbKey, txn::TxnType};

/// ## Transaction header.
/// Contains the transaction header information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxnHeader {
    // Token amount to be transfered
    pub amt: u128,
    pbkey_send: PbKey,
    pbkey_recv: PbKey,
    // The time the txn was created
    pub system_time: u64,
    /// Type of transaction - as int
    pub txn_type: TxnType,
}

// Implement constructor and getters
impl TxnHeader {
    /// Constructor
    pub fn new(
        amt: u128,
        pbkey_send: PbKey,
        pbkey_recv: PbKey,
        system_time: u64,
        txn_type: TxnType,
    ) -> Self {
        Self {
            amt,
            pbkey_send,
            pbkey_recv,
            system_time,
            txn_type,
        }
    }
    /// ### Get `Txn.amt` property
    pub fn amt(&self) -> &u128 {
        &self.amt
    }
    /// ### Get `Txn.pbkey_send` property
    pub fn pbkey_send(&self) -> &PbKey {
        &self.pbkey_send
    }
    /// ### Get `Txn.pbkey_recv` property
    pub fn pbkey_recv(&self) -> &PbKey {
        &self.pbkey_recv
    }
    /// ### Get `Txn.system_time` property
    pub fn system_time(&self) -> &u64 {
        &self.system_time
    }
    /// ### Get `Txn.txn_type` property
    pub fn txn_type(&self) -> &TxnType {
        &self.txn_type
    }
}
