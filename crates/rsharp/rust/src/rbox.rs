#![allow(non_camel_case_types, non_snake_case)]

use crate::{primitives, remangle};
use safer_ffi::prelude::*;

macro_rules! boxes {
    // Base case
    ($path:literal, $t:ty $(,)?) => {
        paste::paste! {
            /// Copies `value` into a rust `Box`
            #[remangle($path)]
            #[ffi_export]
            pub fn [<Box _ $t:camel __new>](value: $t) -> repr_c::Box<$t> {
                repr_c::Box::new(value)
            }

            #[remangle($path)]
            #[ffi_export]
            pub fn [<Box _ $t:camel __drop>](value: repr_c::Box<$t>) {
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

primitives!(; types, boxes, "rsharp");
