mod from_impls;

use crate::contract::properties::channels::Channel;
use crate::contract::properties::dynamic::DynEnum;
use crate::contract::properties::dynamic::{TpPrimitiveType, TpPropertyType};
use crate::contract::properties::primitives;
use crate::contract::ContractDataHandle;
use crate::contract::ObjectHandle;

use derive_more::From;
use paste::paste;

pub use crate::contract::properties::dynamic::__macro::{
    apply_to_channel, apply_to_channel_mut, apply_to_channel_ref,
};

DynEnum!(DynChannel, Channel);

macro_rules! prim_enum_helper {
    ($enum_ident:ident, mut, $($variant:ty),+ $(,)?) => {
        paste! {
            #[derive(From)]
            pub enum $enum_ident<'a> {
                $([<$variant:camel>](&'a mut Channel<$variant>)),+
            }
        }
    };
    ($enum_ident:ident, ref, $($variant:ty),+ $(,)?) => {
        paste! {
            #[derive(Copy, Clone, From)]
            pub enum $enum_ident<'a> {
                $([<$variant:camel>](&'a Channel<$variant>)),+
            }
        }
    };
}

macro_rules! vec_enum_helper {
    ($enum_ident:ident, mut, $($variant:ty),+ $(,)?) => {
        paste!{
            #[derive(From)]
            pub enum $enum_ident<'a> {
                $([<$variant:camel>](&'a mut Channel<Vec<$variant>>)),+
            }
        }
    };
    ($enum_ident:ident, ref, $($variant:ty),+ $(,)?) => {
        paste! {
            #[derive(Copy, Clone, From)]
            pub enum $enum_ident<'a> {
                $([<$variant:camel>](&'a Channel<Vec<$variant>>)),+
            }
        }
    };
}

macro_rules! ref_enum {
    ($enum_ident:ident, ref) => {
        paste! {
            primitives!(; types, prim_enum_helper, [<$enum_ident PrimitiveRef>], ref);
            primitives!(; types, vec_enum_helper, [<$enum_ident VecRef>], ref);

            #[derive(Copy, Clone)]  // to avoid conflict, we will implement `From` manually
            pub enum [<$enum_ident Ref>]<'a> {
                Primitive([<$enum_ident PrimitiveRef>]<'a>),
                Vec([<$enum_ident VecRef>]<'a>),
            }
        }
    };
    ($enum_ident:ident, mut) => {
        paste! {
            primitives!(; types, prim_enum_helper, [<$enum_ident PrimitiveMut>], mut);
            primitives!(; types, vec_enum_helper, [<$enum_ident VecMut>], mut);

            // to avoid conflict, we will implement `From` manually
            pub enum [<$enum_ident Mut>]<'a> {
                Primitive([<$enum_ident PrimitiveMut>]<'a>),
                Vec([<$enum_ident VecMut>]<'a>),
            }
        }
    };
}

ref_enum!(DynChannel, mut);
ref_enum!(DynChannel, ref);

macro_rules! prop_type_helper {
    ($enum_ident:ty, $vec_or_prim:ident, $($variant:ident),+ $(,)?) => {
        impl $enum_ident {
            pub fn prop_type(&self) -> TpPropertyType {
                match self {
                    $(Self::$variant(_) => TpPropertyType::$vec_or_prim(TpPrimitiveType::$variant)),+
                }
            }
        }
    };
}

macro_rules! impl_prop_type {
    ($main_ty:ty, $prim_ty:ty, $vec_ty:ty $(,)?) => {
            primitives!(; idents, prop_type_helper, $prim_ty, Primitive);
            primitives!(; idents, prop_type_helper, $vec_ty, Vec);

            impl $main_ty {
                pub fn prop_type(&self) -> TpPropertyType {
                    match self {
                        Self::Primitive(inner) => inner.prop_type(),
                        Self::Vec(inner) => inner.prop_type(),
                    }
                }
            }
    };
}

impl_prop_type!(DynChannel, DynChannelPrimitive, DynChannelVec);
impl_prop_type!(
    DynChannelRef<'_>,
    DynChannelPrimitiveRef<'_>,
    DynChannelVecRef<'_>
);
impl_prop_type!(
    DynChannelMut<'_>,
    DynChannelPrimitiveMut<'_>,
    DynChannelVecMut<'_>
);
