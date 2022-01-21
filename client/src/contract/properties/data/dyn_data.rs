use super::{private, ITpData};

use crate::contract::ContractId;
use crate::object::ObjectHandle;

use derive_more::From;
use paste::paste;

/// The compile-time type of the ITpData
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TpDataType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Bool,
    F32,
    F64,
    String,
    ObjectHandle,
    ContractId,
    Dynamic,
}

/// A dynamically typed `ITpData` primitive
#[derive(Debug, Clone, PartialEq, From)]
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
    ContractId(ContractId),
}

impl ITpData for DynTpData {
    const DATA_TYPE: TpDataType = TpDataType::Dynamic;
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
    ContractId,
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
}
