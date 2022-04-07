use crate::vec::RustVec;

use std::string::{FromUtf16Error, FromUtf8Error};

/// This is a C-FFI compatible version of a Rust [`String`].
///
/// Its important to note that this type maintains the same invariants as a regular
/// rust `String`. It is unsound to create a `RustString` that doesn't satisfy these
/// invariants.
pub struct RustString {
    data: RustVec<u8>,
}
impl RustString {
    /// # Safety
    /// The caller must guarantee that the `RustVec` contains UTF-8 encoded bytes.
    pub unsafe fn from_utf8_unchecked(data: RustVec<u8>) -> Self {
        Self { data }
    }

    pub fn from_utf8(data: RustVec<u8>) -> Result<Self, FromUtf8Error> {
        String::from_utf8(data.into()).map(Self::from)
    }

    pub fn from_utf16(data: &[u16]) -> Result<Self, FromUtf16Error> {
        String::from_utf16(data.into()).map(Self::from)
    }
}
impl From<String> for RustString {
    fn from(other: String) -> Self {
        Self {
            data: other.into_bytes().into(),
        }
    }
}
impl From<RustString> for String {
    fn from(other: RustString) -> Self {
        // Safety: `RustString`'s buffer upholds all the same invariants as
        // `String`.
        unsafe { String::from_utf8_unchecked(other.data.into()) }
    }
}

pub unsafe extern "C" fn rsharp__RsString__from_utf16(
    str: *const u16,
    len: usize,
) -> *mut RustString {
    assert!(!str.is_null(), "str was null!");
    let data = std::slice::from_raw_parts(str, len);
    let str = RustString::from_utf16(data).expect("The given string was not utf16!");
    Box::into_raw(Box::new(str))
}
