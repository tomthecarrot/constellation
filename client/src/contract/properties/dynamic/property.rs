use super::primitive::TpPrimitiveType;
use super::TpPropertyType;
use crate::contract::properties::dynamic::__macro::DynEnum;
use crate::contract::properties::primitives;
use crate::contract::properties::traits::ITpProperty;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use derive_more::From;
use paste::paste;

DynEnum!(DynTpProperty, DynTpPrimitive, DynTpVec);
impl DynTpProperty {
    pub const fn prop_type(&self) -> TpPropertyType {
        match self {
            Self::Primitive(tpp) => tpp.prop_type(),
            Self::Vec(tpv) => tpv.prop_type(),
        }
    }
}

impl ITpProperty for DynTpProperty {
    fn prop_type(&self) -> TpPropertyType {
        self.prop_type()
    }
}

// ---- DynTpPropertyRef and DynTpPropertyMut ----

macro_rules! dyn_helper {
    ($suffix:ident, $attr:meta, $($maybe_mut:tt)?) => {
        paste! {

            #[$attr]
            pub enum [<DynTpPrimitive $suffix>]<'a> {
                U8(&'a $($maybe_mut)? u8),
                U16(&'a $($maybe_mut)? u16),
                U32(&'a $($maybe_mut)? u32),
                U64(&'a $($maybe_mut)? u64),
                I8(&'a $($maybe_mut)? i8),
                I16(&'a $($maybe_mut)? i16),
                I32(&'a $($maybe_mut)? i32),
                I64(&'a $($maybe_mut)? i64),
                Bool(&'a $($maybe_mut)? bool),
                F32(&'a $($maybe_mut)? f32),
                F64(&'a $($maybe_mut)? f64),
                String(&'a $($maybe_mut)? String),
                ObjectHandle(&'a $($maybe_mut)? ObjectHandle),
                ContractDataHandle(&'a $($maybe_mut)? ContractDataHandle),
            }
            impl [<DynTpPrimitive $suffix>]<'_> {
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
                        Self::ObjectHandle(_) => {
                            TpPropertyType::Primitive(TpPrimitiveType::ObjectHandle)
                        }
                        Self::ContractDataHandle(_) => {
                            TpPropertyType::Primitive(TpPrimitiveType::ContractDataHandle)
                        }
                    }
                }
            }

            #[$attr]
            pub enum [<DynTpVec $suffix>]<'a> {
                U8(&'a $($maybe_mut)? [u8]),
                U16(&'a $($maybe_mut)? [u16]),
                U32(&'a $($maybe_mut)? [u32]),
                U64(&'a $($maybe_mut)? [u64]),
                I8(&'a $($maybe_mut)? [i8]),
                I16(&'a $($maybe_mut)? [i16]),
                I32(&'a $($maybe_mut)? [i32]),
                I64(&'a $($maybe_mut)? [i64]),
                Bool(&'a $($maybe_mut)? [bool]),
                F32(&'a $($maybe_mut)? [f32]),
                F64(&'a $($maybe_mut)? [f64]),
                String(&'a $($maybe_mut)? [String]),
                ObjectHandle(&'a $($maybe_mut)? [ObjectHandle]),
                ContractDataHandle(&'a $($maybe_mut)? [ContractDataHandle]),
            }
            impl [<DynTpVec $suffix>]<'_> {
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
                        Self::ContractDataHandle(_) => {
                            TpPropertyType::Vec(TpPrimitiveType::ContractDataHandle)
                        }
                    }
                }
            }

            #[$attr]
            pub enum [<DynTpProperty $suffix>]<'a> {
                Primitive([<DynTpPrimitive $suffix>]<'a>),
                Vec([<DynTpVec $suffix>]<'a>),
            }
        }
    };
}

dyn_helper!(Ref, derive(Debug, From, PartialEq, Clone),);
dyn_helper!(Mut, derive(Debug, From, PartialEq), mut);

// Maps enum to `TpPrimitiveType` by expanding to match on variants
macro_rules! helper_match {
    ($enum:ident, $enum_type:ident, $($variant:ident),+ $(,)?) => {
        match $enum {
            $(
                $enum_type::$variant(_) => TpPrimitiveType::$variant,
            )*
        }
    };
}

impl<'a> DynTpPropertyMut<'a> {
    pub fn prop_type(&self) -> TpPropertyType {
        use DynTpPrimitiveMut as P;
        use DynTpVecMut as V;
        match self {
            Self::Primitive(p) => {
                TpPropertyType::Primitive(primitives!(idents, helper_match, p, P))
            }
            Self::Vec(v) => TpPropertyType::Vec(primitives!(idents, helper_match, v, V)),
        }
    }
}

impl<'a> DynTpPropertyRef<'a> {
    pub fn prop_type(&self) -> TpPropertyType {
        use DynTpPrimitiveRef as P;
        use DynTpVecRef as V;
        match self {
            Self::Primitive(p) => {
                TpPropertyType::Primitive(primitives!(idents, helper_match, p, P))
            }
            Self::Vec(v) => TpPropertyType::Vec(primitives!(idents, helper_match, v, V)),
        }
    }
}
