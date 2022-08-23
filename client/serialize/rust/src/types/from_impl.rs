use crate::{c, t};

impl From<c::TpPrimitiveType> for t::TpPrimitiveKind {
    fn from(other: c::TpPrimitiveType) -> Self {
        use c::TpPrimitiveType as C;
        use t::TpPrimitiveKind as T;
        match other {
            C::U8 => T::U8,
            C::U16 => T::U16,
            C::U32 => T::U32,
            C::U64 => T::U64,
            C::I8 => T::I8,
            C::I16 => T::I16,
            C::I32 => T::I32,
            C::I64 => T::I64,
            C::Bool => T::Bool,
            C::F32 => T::F32,
            C::F64 => T::F64,
            C::String => T::String,
            C::ObjectHandle => T::ObjectHandle,
            C::ContractDataHandle => T::ContractDataHandle,
        }
    }
}
