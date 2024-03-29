use {
    arrayvec::ArrayString,
    base64::display::Base64Display,
    blake3::{Hash, OUT_LEN},
    serde::{Deserialize, Serialize, Serializer},
    std::{convert::From, fmt},
};

pub type BlakeHex = ArrayString<{ 2 * OUT_LEN }>;

#[derive(Clone, Copy, Hash, Eq, Serialize, Deserialize)]
pub struct BlakeHash([u8; OUT_LEN]);

impl From<blake3::Hash> for BlakeHash {
    fn from(t: blake3::Hash) -> Self {
        BlakeHash(*t.as_bytes())
    }
}
impl From<BlakeHex> for BlakeHash {
    fn from(t: BlakeHex) -> Self {
        let hash = Hash::from_hex(t.as_bytes()).unwrap();
        BlakeHash::from(hash)
    }
}
// Convert bytes to blake hash type
impl From<[u8; OUT_LEN]> for BlakeHash {
    #[inline]
    fn from(bytes: [u8; OUT_LEN]) -> Self {
        Self(bytes)
    }
}
impl BlakeHash {
    pub fn from_bytes(bytes: [u8; OUT_LEN]) -> Self {
        Self(bytes)
    }
    /// The raw bytes of the `Hash`. Note that byte arrays don't provide
    /// constant-time equality checking, so if you need to compare hashes,
    /// prefer the `Hash` type.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; OUT_LEN] {
        &self.0
    }
    /// Encode a `Hash` in lowercase hexadecimal.
    ///
    /// The returned [`ArrayString`] is a fixed size and doesn't allocate memory
    /// on the heap. Note that [`ArrayString`] doesn't provide constant-time
    /// equality checking, so if you need to compare hashes, prefer the `Hash`
    /// type.
    ///
    /// [`ArrayString`]: https://docs.rs/arrayvec/0.5.1/arrayvec/struct.ArrayString.html
    pub fn to_hex(&self) -> BlakeHex {
        let mut s = ArrayString::new();
        let table = b"0123456789abcdef";
        for &b in self.0.iter() {
            s.push(table[(b >> 4) as usize] as char);
            s.push(table[(b & 0xf) as usize] as char);
        }
        s
    }
}
// This implementation is constant-time.
impl PartialEq<[u8; OUT_LEN]> for BlakeHash {
    #[inline]
    fn eq(&self, other: &[u8; OUT_LEN]) -> bool {
        constant_time_eq::constant_time_eq_32(&self.0, other)
    }
}
// This implementation is constant-time.
impl PartialEq for BlakeHash {
    #[inline]
    fn eq(&self, other: &BlakeHash) -> bool {
        constant_time_eq::constant_time_eq_32(&self.0, &other.0)
    }
}

#[allow(clippy::ptr_arg)]
pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
    if s.is_human_readable() {
        s.collect_str(&Base64Display::with_config(v, base64::STANDARD))
    } else {
        serde_bytes::serialize(v, s)
    }
}

impl fmt::Display for BlakeHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Formatting field as `&str` to reduce code size since the `Debug`
        // dynamic dispatch table for `&str` is likely needed elsewhere already,
        // but that for `ArrayString<[u8; 64]>` is not.
        let hex = self.to_hex();
        let hex: &str = hex.as_str();

        f.write_str(hex)
    }
}

impl fmt::Debug for BlakeHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Formatting field as `&str` to reduce code size since the `Debug`
        // dynamic dispatch table for `&str` is likely needed elsewhere already,
        // but that for `ArrayString<[u8; 64]>` is not.
        let hex = self.to_hex();
        let hex: &str = hex.as_str();

        f.debug_tuple("Hash").field(&hex).finish()
    }
}
