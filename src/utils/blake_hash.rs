use arrayvec::ArrayString;
use base64::display::Base64Display;
use serde::{Serialize, Serializer};

pub const OUT_LEN: usize = 32;

#[derive(Clone, Copy, Hash, Debug, Serialize)]
pub struct BlakeHash([u8; OUT_LEN]);

impl BlakeHash {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
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
    pub fn to_hex(&self) -> ArrayString<{ 2 * OUT_LEN }> {
        let mut s = ArrayString::new();
        let table = b"0123456789abcdef";
        for &b in self.0.iter() {
            s.push(table[(b >> 4) as usize] as char);
            s.push(table[(b & 0xf) as usize] as char);
        }
        s
    }
}
/// This implementation is constant-time.
impl PartialEq<[u8; OUT_LEN]> for BlakeHash {
    #[inline]
    fn eq(&self, other: &[u8; OUT_LEN]) -> bool {
        constant_time_eq::constant_time_eq_32(&self.0, other)
    }
}
/// This implementation is constant-time.
impl PartialEq for BlakeHash {
    #[inline]
    fn eq(&self, other: &BlakeHash) -> bool {
        constant_time_eq::constant_time_eq_32(&self.0, &other.0)
    }
}
impl Eq for BlakeHash {}
#[allow(clippy::ptr_arg)]
pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
    if s.is_human_readable() {
        s.collect_str(&Base64Display::with_config(v, base64::STANDARD))
    } else {
        serde_bytes::serialize(v, s)
    }
}
