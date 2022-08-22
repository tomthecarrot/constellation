#![allow(non_camel_case_types, non_snake_case)]

use crate::remangle;
use crate::value_types::value_types;
use safer_ffi::prelude::*;

macro_rules! boxes {
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
        boxes!($path, $first_t);
        boxes!($path, $($tail_t),+);
    };
}

value_types!(; types, boxes, "rsharp");
