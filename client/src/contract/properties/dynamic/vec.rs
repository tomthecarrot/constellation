use super::__macro::DynEnum;
use super::primitive::TpPrimitiveType;
use super::property::TpPropertyType;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use paste::paste;

DynEnum!(DynTpVec, Vec);
impl DynTpVec {
    pub const fn prop_type(&self) -> TpPropertyType {
        match self {
            Self::U8(_) => TpPropertyType::Vec(TpPrimitiveType::U8),
            Self::U16(_) => TpPropertyType::Vec(TpPrimitiveType::U16),
            Self::U32(_) => TpPropertyType::Vec(TpPrimitiveType::U32),
            Self::U64(_) => TpPropertyType::Vec(TpPrimitiveType::U64),
            Self::I8(_) => TpPropertyType::Vec(TpPrimitiveType::I8),
            Self::I16(_) => TpPropertyType::Vec(TpPrimitiveType::I16),
            Self::I32(_) => TpPropertyType::Vec(TpPrimitiveType::I32),
            Self::I64(_) => TpPropertyType::Vec(TpPrimitiveType::I64),
            Self::Bool(_) => TpPropertyType::Vec(TpPrimitiveType::Bool),
            Self::F32(_) => TpPropertyType::Vec(TpPrimitiveType::F32),
            Self::F64(_) => TpPropertyType::Vec(TpPrimitiveType::F64),
            Self::String(_) => TpPropertyType::Vec(TpPrimitiveType::String),
            Self::ObjectHandle(_) => TpPropertyType::Vec(TpPrimitiveType::ObjectHandle),
            Self::ContractDataHandle(_) => TpPropertyType::Vec(TpPrimitiveType::ContractDataHandle),
        }
    }
}

macro_rules! impl_equality {
    // base case
    ($t:ty) => {
        paste! {
            impl PartialEq<Vec<$t>> for DynTpVec {
                fn eq(&self, other: &Vec<$t>) -> bool {
                    if let Self::[<$t:camel>](inner) = self {
                        inner == other
                    } else {
                        false
                    }
                }
            }

            impl PartialEq<DynTpVec> for Vec<$t> {
                fn eq(&self, other: &DynTpVec) -> bool {
                    if let DynTpVec::[<$t:camel>](other) = other {
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
        impl_equality!($t);
        impl_equality!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_equality!($($tail),+);
    };
}

impl_equality!(
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
