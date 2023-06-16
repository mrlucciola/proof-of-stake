use crate::ledger::{
    general::PbKey,
    txn::{Txn, TxnHeader, TxnId, TxnSignature, TxnType},
    txn_pool::TxnMapKey,
};
use std::borrow::BorrowMut;

impl Txn {
    /// ### Get property `Txn.id`.
    /// Panic when accessing unset value.
    pub fn id(&self) -> &TxnId {
        self.id.as_ref().unwrap()
    }
    /// ### Get `TxnMap` key type (derived from TxnId).
    pub fn id_key(&self) -> TxnMapKey {
        self.id.as_ref().unwrap().into()
    }
    /// ### Get property `Txn.signature`.
    pub fn signature(&self) -> &TxnSignature {
        self.signature.as_ref().unwrap()
    }
    /// ### Get property `Txn.header`.
    pub fn header(&self) -> &TxnHeader {
        &self.header
    }
    /// ### Get property `Txn.header` as mutable.
    pub fn header_mut(&mut self) -> &mut TxnHeader {
        self.header.borrow_mut()
    }

    // Getters for header properties
    /// ### Get property `Txn.amt`.
    pub fn amt(&self) -> &u128 {
        self.header.amt()
    }
    /// ### Get property `Txn.pbkey_send`.
    pub fn pbkey_send(&self) -> &PbKey {
        self.header.pbkey_send()
    }
    /// ### Get property `Txn.pbkey_recv`.
    pub fn pbkey_recv(&self) -> &PbKey {
        self.header.pbkey_recv()
    }
    /// ### Get property `Txn.system_time`.
    pub fn system_time(&self) -> &u64 {
        self.header.system_time()
    }
    /// ### Get property `Txn.txn_type`.
    pub fn txn_type(&self) -> &TxnType {
        self.header.txn_type()
    }
}
