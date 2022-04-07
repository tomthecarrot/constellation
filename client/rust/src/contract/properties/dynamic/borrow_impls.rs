use super::property::{
    DynTpPrimitive, DynTpPrimitiveMut, DynTpPrimitiveRef, DynTpProperty, DynTpPropertyMut,
    DynTpPropertyRef, DynTpVec, DynTpVecMut, DynTpVecRef,
};
use crate::contract::properties::primitives;

use better_borrow::{BBorrow, BBorrowMut};

impl<'b> BBorrow<'b, DynTpPropertyRef<'b>> for DynTpProperty {
    fn borrow<'a>(&'a self) -> DynTpPropertyRef<'b>
    where
        'a: 'b,
    {
        macro_rules! helper_primitive {
            ($enum:ident, $($ident:ident),+ $(,)?) => {
                match $enum {
                    $(
                        DynTpPrimitive::$ident(inner) => DynTpPrimitiveRef::from(inner).into(),
                    )+
                }
            };
        }
        macro_rules! helper_vec {
            ($enum:ident, $($ident:ident),+ $(,)?) => {
                match $enum {
                    $(
                        DynTpVec::$ident(inner) => DynTpVecRef::from(inner.as_slice()).into(),
                    )+
                }
            };
        }
        match self {
            Self::Primitive(prim) => primitives!(idents, helper_primitive, prim),
            Self::Vec(vec) => primitives!(idents, helper_vec, vec),
        }
    }
}

impl<'b> BBorrowMut<'b, DynTpPropertyMut<'b>> for DynTpProperty {
    type Borrowed = DynTpPropertyRef<'b>;

    fn borrow_mut<'a>(&'a mut self) -> DynTpPropertyMut<'b>
    where
        'a: 'b,
    {
        macro_rules! helper_primitive {
            ($enum:ident, $($ident:ident),+ $(,)?) => {
                match $enum {
                    $(
                        DynTpPrimitive::$ident(inner) => DynTpPrimitiveMut::from(inner).into(),
                    )+
                }
            };
        }
        macro_rules! helper_vec {
            ($enum:ident, $($ident:ident),+ $(,)?) => {
                match $enum {
                    $(
                        DynTpVec::$ident(inner) => DynTpVecMut::from(inner.as_mut_slice()).into(),
                    )+
                }
            };
        }
        match self {
            Self::Primitive(prim) => primitives!(idents, helper_primitive, prim),
            Self::Vec(vec) => primitives!(idents, helper_vec, vec),
        }
    }
}

// ---- Also impl for ref types ----

impl<'b> BBorrow<'b, DynTpPropertyRef<'b>> for &DynTpProperty {
    fn borrow<'a>(&'a self) -> DynTpPropertyRef<'b>
    where
        'a: 'b,
    {
        BBorrow::borrow(*self)
    }
}

impl<'b> BBorrowMut<'b, DynTpPropertyMut<'b>> for &mut DynTpProperty {
    type Borrowed = &'b DynTpProperty;

    fn borrow_mut<'a>(&'a mut self) -> DynTpPropertyMut<'b>
    where
        'a: 'b,
    {
        BBorrowMut::borrow_mut(*self)
    }
}
