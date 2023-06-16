use crate::ledger::{
    block::{constants::BLOCK_SIGNATURE_CTX, error::BlockError, types::BlockDigest, Block},
    general::PbKey,
};

impl Block {
    /// ### Check if signature is valid.
    ///
    /// 1. Assert there is a signature (not applicable currently - program panics)
    /// 1. Assert signature is valid
    ///
    /// Return is a Result of an Option to handle non-existant signatures
    /// - `Some()` indicates a signature exists and its valid/invalid
    /// - `None` indicates there is no signature
    /// - `Error` is for error handling
    pub fn is_signature_valid(&self, signer_pbkey: &PbKey) -> std::result::Result<(), BlockError> {
        // create message for verification
        let msg: BlockDigest = self.calc_id().into();
        let mut presigned_msg = BLOCK_SIGNATURE_CTX.to_vec();
        presigned_msg.append(&mut msg.to_vec());

        // get the current signature
        let block_signature = self.signature();
        let sig_test = ed25519::Signature::from_bytes(&block_signature.0).unwrap();
        let signer_conv: ed25519_dalek::PublicKey = signer_pbkey.into();

        match signer_conv.verify_strict(&presigned_msg, &sig_test) {
            Ok(_) => Ok(()),
            Err(e) => Err(BlockError::InvalidSignature(e, sig_test)),
        }
    }

    /// ### Check if block is valid.
    ///
    /// Valid criteria:
    ///   - all struct properties are not `None` - (not applicable - default is panic)
    ///   - hash is valid
    ///   - signature is valid
    pub fn is_valid(&self, signer_pbkey: &PbKey) -> std::result::Result<(), BlockError> {
        // validate fields
        // validate hash
        if self.calc_id() != self.id() {
            return Err(BlockError::IncorrectId);
        }
        // validate signature
        self.is_signature_valid(&signer_pbkey)?;

        Ok(())
    }
}
