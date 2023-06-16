// local
use super::{Txn, TxnId, TxnSignature, TxnType};
use crate::ledger::{general::PbKey, txn_pool::TxnMapKey};

impl Txn {
    /// ### Get `Txn.id` property.
    /// Panic when accessing unset value.
    pub fn id(&self) -> &TxnId {
        self.id.as_ref().unwrap()
    }
    /// ### Get `TxnMap` key type (derived from TxnId).
    pub fn id_key(&self) -> TxnMapKey {
        self.id.as_ref().unwrap().into()
    }
    /// ### Getter for `Txn` `signature` property
    pub fn signature(&self) -> &TxnSignature {
        self.signature.as_ref().unwrap()
    }

    // Getters for header properties
    /// ### Get `Txn.amt` property
    pub fn amt(&self) -> &u128 {
        self.header.amt()
    }
    /// ### Get `Txn.pbkey_send` property
    pub fn pbkey_send(&self) -> &PbKey {
        self.header.pbkey_send()
    }
    /// ### Get `Txn.pbkey_recv` property
    pub fn pbkey_recv(&self) -> &PbKey {
        self.header.pbkey_recv()
    }
    /// ### Get `Txn.system_time` property
    pub fn system_time(&self) -> &u64 {
        self.header.system_time()
    }
    /// ### Get `Txn.txn_type` property
    pub fn txn_type(&self) -> &TxnType {
        self.header.txn_type()
    }
}
