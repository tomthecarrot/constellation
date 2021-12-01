use super::{private, ITpData};

use crate::contract::ContractHandle;
use crate::object::ObjectHandle;

use enum_dispatch::enum_dispatch;
use paste::paste;

/// A dynamically typed `ITpData` primitive
#[enum_dispatch(ITpData)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DynTpData {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Bool(bool),
    F32(f32),
    F64(f64),
    String(String),
    ObjectHandle(ObjectHandle),
    ContractHandle(ContractHandle),
}

impl private::Sealed for DynTpData {}

// ---- PartialEq and PartialOrd impls ----

macro_rules! impl_dyntpdata {
    // base case
    ($t:ty) => {
        paste! {
            impl PartialEq<$t> for DynTpData {
                fn eq(&self, other: &$t) -> bool {
                    if let Self::[<$t:camel>](inner) = self {
                        inner == other
                    } else {
                        false
                    }
                }
            }

            impl PartialEq<DynTpData> for $t {
                fn eq(&self, other: &DynTpData) -> bool {
                    if let DynTpData::[<$t:camel>](other) = other {
                        self == other
                    } else {
                        false
                    }
                }
            }

            impl PartialOrd<$t> for DynTpData {
                fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                    match self {
                        Self::[<$t:camel>](inner) => inner.partial_cmp(other),
                        _ => None,
                    }
                }
            }

            impl PartialOrd<DynTpData> for $t {
                fn partial_cmp(&self, other: &DynTpData) -> Option<std::cmp::Ordering> {
                    match other {
                        DynTpData::[<$t:camel>](other) => self.partial_cmp(other),
                        _ => None,
                    }
                }
            }
        }
    };
    // recursive case
    ($t:ty, $($tail:ty),+) => {
        impl_dyntpdata!($t);
        impl_dyntpdata!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_dyntpdata!($($tail),+);
    };
}

impl_dyntpdata!(
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    bool,
    f32,
    f64,
    String,
    ObjectHandle,
    ContractHandle,
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partialeq() {
        let u16_1337 = 1337u16;
        let u16_7331 = 7331u16;
        let u32_1337 = 1337u32;
        let u32_7331 = 7331u32;
        let dyn_1337 = DynTpData::from(1337u16);

        // Compare same type & value
        assert_eq!(dyn_1337, u16_1337);
        assert_eq!(u16_1337, dyn_1337);

        // Compare mismatched type, same val
        assert_ne!(dyn_1337, u32_1337);
        assert_ne!(u32_1337, dyn_1337);

        // Compare same type, mismatched val
        assert_ne!(dyn_1337, u16_7331);
        assert_ne!(u16_7331, dyn_1337);

        // Compare mismatched type and val
        assert_ne!(dyn_1337, u32_7331);
        assert_ne!(u32_7331, dyn_1337);
    }

    #[test]
    fn test_partialord() {
        use std::cmp::Ordering;

        let u16_1337 = 1337u16;
        let u16_7331 = 7331u16;
        let u32_1337 = 1337u32;
        let u32_7331 = 7331u32;
        let dyn_1337 = DynTpData::from(1337u16);

        // Compare same type & value
        assert_eq!(dyn_1337.partial_cmp(&u16_1337), Some(Ordering::Equal));
        assert_eq!(u16_1337.partial_cmp(&dyn_1337), Some(Ordering::Equal));

        // Compare mismatched type, same val
        assert_ne!(dyn_1337, u32_1337);
        assert_ne!(u32_1337, dyn_1337);
        assert_eq!(dyn_1337.partial_cmp(&u32_1337), None);
        assert_eq!(u32_1337.partial_cmp(&dyn_1337), None);

        // Compare same type, mismatched val
        assert_ne!(dyn_1337, u16_7331);
        assert_ne!(u16_7331, dyn_1337);
        assert!(dyn_1337 < u16_7331);
        assert!(u16_7331 > dyn_1337);
        assert!(dyn_1337 > 1u16);
        assert!(1u16 < dyn_1337);

        // Compare mismatched type and val
        assert_eq!(dyn_1337.partial_cmp(&u32_7331), None);
        assert_eq!(u32_7331.partial_cmp(&dyn_1337), None);
    }
}
