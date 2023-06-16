use super::constants::{TXN_DIGEST_LEN, TXN_SIGNATURE_CTX};

pub type TxnDigest = [u8; 64];
pub type TxnCtxDigest = [u8; TXN_SIGNATURE_CTX.len() + TXN_DIGEST_LEN];
