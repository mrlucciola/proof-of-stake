use super::{Txn, TxnId, TxnSignature};

impl Txn {
    /// ## Get identifier (hash) for txn and set on txn object and store the output on the Txn object
    /// Returns id.
    pub fn set_id(&mut self) -> TxnId {
        let id = self.calc_id();
        self.id = Some(id);

        id
    }
    /// ## Set the signature for the transaction.
    pub fn set_signature(&mut self, signature: TxnSignature) {
        self.signature = Some(signature);
    }
}
