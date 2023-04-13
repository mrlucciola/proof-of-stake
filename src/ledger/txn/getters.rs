// local
use super::{Txn, TxnId, TxnSignature};
use crate::ledger::{general::PbKey, txn_pool::TxnMapKey};

impl Txn {
    /// ### Get `Txn.id` property.
    /// Panic when accessing unset value.
    pub fn id(&self) -> TxnId {
        self.id.unwrap()
    }
    /// ### Get `TxnMap` key type (derived from TxnId).
    /// @todo change to byte array
    pub fn id_key(&self) -> TxnMapKey {
        self.id().into()
    }
    /// ### Getter for `Txn` `signature` property
    pub fn signature(&self) -> &TxnSignature {
        self.signature.as_ref().unwrap()
    }
    pub fn pbkey_send(&self) -> &PbKey {
        &self.pbkey_send.into()
    }
    pub fn pbkey_recv(&self) -> &PbKey {
        // PbKey::from_bytes(&self.pbkey_recv).unwrap()
        &self.pbkey_recv.into()
    }
}
