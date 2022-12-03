use crate::ledger::{blocks::BlockId, general::PvKey, txn::TxnId};
use secp256k1::{Message, Secp256k1};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SecpEcdsaSignature(pub(crate) secp256k1::ecdsa::Signature);
impl Into<SecpEcdsaSignature> for secp256k1::ecdsa::Signature {
    fn into(self) -> SecpEcdsaSignature {
        SecpEcdsaSignature(self)
    }
}
impl SecpEcdsaSignature {
    // Serializes the signature in compact format
    pub fn serialize_compact(&self) -> [u8; 64] {
        secp256k1::ecdsa::Signature::serialize_compact(&self.0)
    }
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
impl From<SecpEcdsaSignature> for secp256k1::ecdsa::Signature {
    fn from(x: SecpEcdsaSignature) -> secp256k1::ecdsa::Signature {
        x.0
    }
}
// impl fmt::Display for SecpEcdsaSignature {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Formatting field as `&str` to reduce code size since the `Debug`
//         // dynamic dispatch table for `&str` is likely needed elsewhere already,
//         // but that for `ArrayString<[u8; 64]>` is not.
//         f.write_str(&self.to_string())
//     }
// }

#[derive(Debug, Clone)]
pub struct BlockSignatureSecpEcdsa(pub(crate) SecpEcdsaSignature);
#[derive(Debug, Clone)]
pub struct TxnSignatureSecpEcdsa(pub(crate) SecpEcdsaSignature);

impl Into<BlockSignatureSecpEcdsa> for SecpEcdsaSignature {
    fn into(self) -> BlockSignatureSecpEcdsa {
        BlockSignatureSecpEcdsa(self)
    }
}
impl Into<TxnSignatureSecpEcdsa> for SecpEcdsaSignature {
    fn into(self) -> TxnSignatureSecpEcdsa {
        TxnSignatureSecpEcdsa(self)
    }
}
impl Into<TxnSignatureSecpEcdsa> for secp256k1::ecdsa::Signature {
    fn into(self) -> TxnSignatureSecpEcdsa {
        TxnSignatureSecpEcdsa(SecpEcdsaSignature(self))
    }
}
impl SecpEcdsaSignatureTrait for TxnSignatureSecpEcdsa {}
impl SecpEcdsaSignatureTrait for BlockSignatureSecpEcdsa {}
impl TxnSignatureSecpEcdsa {
    pub fn msg_from_id(txn_id: &TxnId) -> Message {
        secp256k1::Message::from_slice(txn_id.as_bytes()).unwrap()
    }
    pub fn sign_id(txn_id: &TxnId, pv_key: &PvKey) -> TxnSignatureSecpEcdsa {
        let msg = Self::msg_from_id(txn_id);
        Self::sign_msg(&msg, pv_key).into()
    }
}
impl BlockSignatureSecpEcdsa {
    pub fn msg_from_id(block_id: &BlockId) -> Message {
        secp256k1::Message::from_slice(block_id.as_bytes()).unwrap()
    }
    pub fn sign_id(block_id: &BlockId, pv_key: &PvKey) -> BlockSignatureSecpEcdsa {
        let msg = Self::msg_from_id(block_id);
        Self::sign_msg(&msg, pv_key).into()
    }
}

pub trait SecpEcdsaSignatureFxn {
    fn msg_from_bytes(bytes: &[u8]) -> Message;
    fn sign_msg(msg: &secp256k1::Message, pv_key: &PvKey) -> SecpEcdsaSignature;
    fn sign_bytes(bytes: &[u8], pv_key: &PvKey) -> SecpEcdsaSignature;
    // fn to_string(&self) -> String;
}

// trait Test {}

impl<T> SecpEcdsaSignatureFxn for T
where
    T: SecpEcdsaSignatureTrait,
{
    fn msg_from_bytes(bytes: &[u8]) -> Message {
        secp256k1::Message::from_slice(bytes).unwrap()
    }
    fn sign_msg(msg: &secp256k1::Message, pv_key: &PvKey) -> SecpEcdsaSignature {
        let secp = Secp256k1::new();

        secp.sign_ecdsa(&msg, pv_key).into()
    }

    fn sign_bytes(bytes: &[u8], pv_key: &PvKey) -> SecpEcdsaSignature {
        let msg = Self::msg_from_bytes(bytes);

        Self::sign_msg(&msg, pv_key)
    }
    // fn to_string(&self) -> String {}
}
pub trait SecpEcdsaSignatureTrait {}

pub type TxnSignature = TxnSignatureSecpEcdsa;
pub type BlockSignature = BlockSignatureSecpEcdsa;
