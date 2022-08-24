#![allow(non_camel_case_types, non_snake_case)]

use crate::opaque_types::opaque_types;
use crate::remangle;
use crate::string::String;
use crate::value_types::value_types;
use safer_ffi::prelude::*;

macro_rules! boxes_value {
    // Base case
    ($path:literal, $t:ty $(,)?) => {
        paste::paste! {

            #[remangle($path)]
            #[ffi_export]
            pub fn [<Box _ $t:camel __new>](value: $t) -> repr_c::Box<crate::value_types::$t> {
                repr_c::Box::new(value)
            }

            #[remangle($path)]
            #[ffi_export]
            pub fn [<Box _ $t:camel __drop>](value: repr_c::Box<crate::value_types::$t>) {
                drop(value)
            }
        }
    };
    // recursive case
    ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
        boxes_value!($path, $first_t);
        boxes_value!($path, $($tail_t),+);
    };
}

macro_rules! boxes_opaque {
    // Base case
    ($path:literal, $t:ty $(,)?) => {
        paste::paste! {

            #[remangle($path)]
            #[ffi_export]
            pub fn [<Box _ $t:camel __new>](value: $t) -> repr_c::Box<crate::opaque_types::$t> {
                repr_c::Box::new(value)
            }

            #[remangle($path)]
            #[ffi_export]
            pub fn [<Box _ $t:camel __drop>](value: repr_c::Box<crate::opaque_types::$t>) {
                drop(value)
            }
        }
    };
    // recursive case
    ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
        boxes_opaque!($path, $first_t);
        boxes_opaque!($path, $($tail_t),+);
    };
}

value_types!(; types, boxes_value, "rsharp");
opaque_types!(; types, boxes_opaque, "rsharp");
