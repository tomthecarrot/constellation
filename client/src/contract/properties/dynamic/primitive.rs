use super::__macro::DynEnum;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use paste::paste;

use super::property::TpPropertyType;

/// The static type of the ITpData
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TpPrimitiveType {
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
    ContractDataHandle,
}

DynEnum!(DynTpPrimitive);

impl DynTpPrimitive {
    pub const fn prop_type(&self) -> TpPropertyType {
        match self {
            Self::U8(_) => TpPropertyType::Primitive(TpPrimitiveType::U8),
            Self::U16(_) => TpPropertyType::Primitive(TpPrimitiveType::U16),
            Self::U32(_) => TpPropertyType::Primitive(TpPrimitiveType::U32),
            Self::U64(_) => TpPropertyType::Primitive(TpPrimitiveType::U64),
            Self::I8(_) => TpPropertyType::Primitive(TpPrimitiveType::I8),
            Self::I16(_) => TpPropertyType::Primitive(TpPrimitiveType::I16),
            Self::I32(_) => TpPropertyType::Primitive(TpPrimitiveType::I32),
            Self::I64(_) => TpPropertyType::Primitive(TpPrimitiveType::I64),
            Self::Bool(_) => TpPropertyType::Primitive(TpPrimitiveType::Bool),
            Self::F32(_) => TpPropertyType::Primitive(TpPrimitiveType::F32),
            Self::F64(_) => TpPropertyType::Primitive(TpPrimitiveType::F64),
            Self::String(_) => TpPropertyType::Primitive(TpPrimitiveType::String),
            Self::ObjectHandle(_) => TpPropertyType::Primitive(TpPrimitiveType::ObjectHandle),
            Self::ContractDataHandle(_) => {
                TpPropertyType::Primitive(TpPrimitiveType::ContractDataHandle)
            }
        }
    }
}

// ---- PartialEq and PartialOrd impls ----

macro_rules! impl_dyntpdata {
    // base case
    ($t:ty) => {
        paste! {
            impl PartialEq<$t> for DynTpPrimitive {
                fn eq(&self, other: &$t) -> bool {
                    if let Self::[<$t:camel>](inner) = self {
                        inner == other
                    } else {
                        false
                    }
                }
            }

            impl PartialEq<DynTpPrimitive> for $t {
                fn eq(&self, other: &DynTpPrimitive) -> bool {
                    if let DynTpPrimitive::[<$t:camel>](other) = other {
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
    ContractDataHandle,
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
        let dyn_1337 = DynTpPrimitive::from(1337u16);

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
