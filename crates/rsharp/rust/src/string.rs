#![allow(non_snake_case)]

use crate::remangle;
use derive_more::{From, Into};
use ref_cast::RefCast;
use safer_ffi::prelude::*;

#[derive_ReprC]
#[ReprC::opaque]
#[repr(transparent)]
#[derive(RefCast, From, Into, Debug, Clone, Eq, PartialEq, Hash)]
pub struct String {
    inner: std::string::String,
}

#[remangle("rsharp")]
#[ffi_export]
pub fn String__value(s: &String) -> c_slice::Ref<u8> {
    s.inner.as_bytes().into()
}

#[remangle("rsharp")]
#[ffi_export]
pub fn String__drop(s: repr_c::Box<String>) {
    drop(s)
}

#[remangle("rsharp")]
#[ffi_export]
pub fn String__copy_utf8(utf8: c_slice::Ref<u8>) -> repr_c::Box<String> {
    let utf8 = utf8.as_slice();
    repr_c::Box::new(
        std::string::String::from_utf8(utf8.to_vec())
            .expect("Invalid utf8!")
            .into(),
    )
}

#[remangle("rsharp")]
#[ffi_export]
pub fn String__copy_utf16(utf16: c_slice::Ref<u16>) -> repr_c::Box<String> {
    let utf16 = utf16.as_slice();
    repr_c::Box::new(
        std::string::String::from_utf16(utf16)
            .expect("Invalid utf16")
            .into(),
    )
}

// ---- From trait impls ----

impl<'a> From<&'a std::string::String> for &'a String {
    fn from(from: &'a std::string::String) -> &'a String {
        String::ref_cast(from)
    }
}
impl<'a> From<&'a mut std::string::String> for &'a mut String {
    fn from(from: &'a mut std::string::String) -> &'a mut String {
        String::ref_cast_mut(from)
    }
}
impl<'a> From<&'a String> for &'a std::string::String {
    fn from(from: &'a String) -> &'a std::string::String {
        &from.inner
    }
}
impl<'a> From<&'a mut String> for &'a mut std::string::String {
    fn from(from: &'a mut String) -> &'a mut std::string::String {
        &mut from.inner
    }
}
