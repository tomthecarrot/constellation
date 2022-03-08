use super::primitive::TpPrimitiveType;
use super::TpPropertyType;
use crate::contract::properties::dynamic::__macro::DynEnum;
use crate::contract::properties::primitives;
use crate::contract::properties::traits::ITpProperty;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use derive_more::From;
use paste::paste;

DynEnum!(
    DynTpProperty,
    DynTpPrimitive,
    DynTpVec | derive(Clone, PartialEq)
);
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

// Used to generate Primitive variant enum
macro_rules! prim_enum_helper {
    ($enum_ident:ident, mut, $($variant_type:ty),+ $(,)?) => {
        paste! {
            #[derive(Debug, From, PartialEq)]
            pub enum $enum_ident<'a> {
                $([<$variant_type:camel>](&'a mut $variant_type)),+
            }
        }
    };
    ($enum_ident:ident, $($variant_type:ty),+ $(,)?) => {
        paste! {
            #[derive(Debug, From, PartialEq)]
            pub enum $enum_ident<'a> {
                $([<$variant_type:camel>](&'a $variant_type)),+
            }
        }
    };
}
#[doc(hidden)]
pub(in crate::contract::properties) use prim_enum_helper;

// Used to generate Vec variant enum
macro_rules! vec_enum_helper {
    ($enum_ident:ident, mut, $($variant_type:ty),+ $(,)?) => {
        paste! {
            #[derive(Debug, From, PartialEq)]
            pub enum $enum_ident<'a> {
                $([<$variant_type:camel>](&'a mut [$variant_type])),+
            }
        }
    };
    ($enum_ident:ident, $($variant_type:ty),+ $(,)?) => {
        paste! {
            #[derive(Debug, From, PartialEq)]
            pub enum $enum_ident<'a> {
                $([<$variant_type:camel>](&'a [$variant_type])),+
            }
        }
    };
}
#[doc(hidden)]
pub(in crate::contract::properties) use vec_enum_helper;

// Used in the `prop_type() -> TpPropertyType` function
macro_rules! prop_type_helper {
    ($enum:expr, $prop_type:ident, $($variant:ident),+ $(,)?) => {
        match $enum {
            $(Self::$variant(_) => TpPropertyType::$prop_type(TpPrimitiveType::$variant)),+
        }
    };
}
#[doc(hidden)]
pub(in crate::contract::properties) use prop_type_helper;

// Generates all relevant enums and enum impls for a borrowed version of `DynTpProperty`
macro_rules! DynEnumRef {
    ($prefix:ident, $suffix:ident, $attr:meta$(, $maybe_mut:tt)? $(,)?) => {
        paste! {

            // Primitive variant
            primitives!(; types, prim_enum_helper, [<$prefix Primitive $suffix>]$(, $maybe_mut)?);
            impl [<$prefix Primitive $suffix>]<'_> {
                pub const fn prop_type(&self) -> TpPropertyType {
                    use $crate::contract::properties::dynamic::property::prop_type_helper;
                    primitives!(idents, prop_type_helper, self, Primitive)
                }
            }

            // Vec variant
            primitives!(; types, vec_enum_helper, [<$prefix Vec $suffix>]$(, $maybe_mut)?);
            impl [<$prefix Vec $suffix>]<'_> {
                pub const fn prop_type(&self) -> TpPropertyType {
                    use $crate::contract::properties::dynamic::property::prop_type_helper;
                    primitives!(idents, prop_type_helper, self, Vec)
                }
            }

            // Main enum
            #[$attr]
            pub enum [<$prefix Property $suffix>]<'a> {
                Primitive([<$prefix Primitive $suffix>]<'a>),
                Vec([<$prefix Vec $suffix>]<'a>),
            }
            impl [<$prefix Property $suffix>]<'_> {
                pub fn prop_type(&self) -> TpPropertyType {
                    match self {
                        Self::Primitive(p) => p.prop_type(),
                        Self::Vec(v) => v.prop_type(),
                    }
                }
            }
        }
    };
}

// Create `DynTpPropertyRef`
DynEnumRef!(DynTp, Ref, derive(Debug, From, PartialEq, Clone, Copy));
// Create `DynTpPropertyMut`
DynEnumRef!(DynTp, Mut, derive(Debug, From, PartialEq), mut);

// ---- Impl missing Clone and Copy traits on DynTp*Ref ----

macro_rules! clone_helper {
    ($enum:expr, $($variant:ident),+ $(,)?) => {
        match $enum {
            $(Self::$variant(inner) => Self::from(inner)),+
        }
    }
}

impl Clone for DynTpPrimitiveRef<'_> {
    fn clone(&self) -> Self {
        primitives!(idents, clone_helper, *self)
    }
}
impl Clone for DynTpVecRef<'_> {
    fn clone(&self) -> Self {
        primitives!(idents, clone_helper, *self)
    }
}

impl Copy for DynTpPrimitiveRef<'_> {}
impl Copy for DynTpVecRef<'_> {}
