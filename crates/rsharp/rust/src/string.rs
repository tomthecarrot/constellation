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
