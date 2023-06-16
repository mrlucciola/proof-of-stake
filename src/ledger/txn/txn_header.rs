use crate::ledger::{general::PbKey, txn::TxnType};
use serde::{Deserialize, Serialize};

/// ## Transaction header.
/// Contains the transaction header information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxnHeader {
    // Token amount to be transfered
    amt: u128,
    pbkey_send: PbKey,
    pbkey_recv: PbKey,
    // The time the txn was created
    system_time: u64,
    /// Type of transaction - as int
    txn_type: TxnType,
}

// Implement constructor and getters
impl TxnHeader {
    /// ### Create a new transaction header.
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
    /// ### Get property `Txn.amt`.
    pub fn amt(&self) -> &u128 {
        &self.amt
    }
    /// ### Get property `Txn.pbkey_send`.
    pub fn pbkey_send(&self) -> &PbKey {
        &self.pbkey_send
    }
    /// ### Get property `Txn.pbkey_recv`.
    pub fn pbkey_recv(&self) -> &PbKey {
        &self.pbkey_recv
    }
    /// ### Get property `Txn.system_time`.
    pub fn system_time(&self) -> &u64 {
        &self.system_time
    }
    /// ### Get property `Txn.txn_type`.
    pub fn txn_type(&self) -> &TxnType {
        &self.txn_type
    }

    /// ### Serialize transaction header to bytes.
    ///
    /// @todo replace `Vec<u8>` - don't allocate if all inputs have known size.
    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Error serializing transaction header")
    }
}
