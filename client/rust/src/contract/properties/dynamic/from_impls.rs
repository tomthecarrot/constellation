use super::property::{
    DynTpPrimitiveMut, DynTpPrimitiveRef, DynTpProperty, DynTpPropertyMut, DynTpPropertyRef,
    DynTpVecMut, DynTpVecRef,
};
use crate::contract::properties::primitives;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use better_borrow::{BBorrow, BBorrowMut};

impl<'a> From<&'a DynTpProperty> for DynTpPropertyRef<'a> {
    fn from(other: &'a DynTpProperty) -> Self {
        BBorrow::borrow(other)
    }
}

impl<'a> From<&'a mut DynTpProperty> for DynTpPropertyMut<'a> {
    fn from(other: &'a mut DynTpProperty) -> Self {
        BBorrowMut::borrow_mut(other)
    }
}

macro_rules! impl_from {
    // base case
    ($t:ty) => {

        // ---- impl special case for Vec -> DynTpVecRef/Mut ----
        impl <'a> From<&'a Vec<$t>> for DynTpVecRef<'a> {
            fn from(other: &'a Vec<$t>) -> Self {
                DynTpVecRef::from(other.as_slice())
            }
        }

        impl <'a> From<&'a mut Vec<$t>> for DynTpVecMut<'a> {
            fn from(other: &'a mut Vec<$t>) -> Self {
                DynTpVecMut::from(other.as_mut_slice())
            }
        }

        // ---- Impl conversion from primitive to property ----
        impl <'a> From<&'a $t> for DynTpPropertyRef<'a> {
            fn from(other: &'a $t) -> Self {
                DynTpPrimitiveRef::from(other).into()
            }
        }
        impl <'a> From<&'a mut $t> for DynTpPropertyMut<'a> {
            fn from(other: &'a mut $t) -> Self {
                DynTpPrimitiveMut::from(other).into()
            }
        }

        // ---- Impl conversion from vec to property ----
        impl <'a> From<&'a Vec<$t>> for DynTpPropertyRef<'a> {
            fn from(other: &'a Vec<$t>) -> Self {
                DynTpVecRef::from(other).into()
            }
        }

        impl <'a> From<&'a mut Vec<$t>> for DynTpPropertyMut<'a> {
            fn from(other: &'a mut Vec<$t>) -> Self {
                DynTpVecMut::from(other).into()
            }
        }

        // ---- Impl conversion from slice to property ----
        impl <'a> From<&'a [$t]> for DynTpPropertyRef<'a> {
            fn from(other: &'a [$t]) -> Self {
                DynTpVecRef::from(other).into()
            }
        }

        impl <'a> From<&'a mut [$t]> for DynTpPropertyMut<'a> {
            fn from(other: &'a mut [$t]) -> Self {
                DynTpVecMut::from(other).into()
            }
        }
    };

    // recursive case
    ($t:ty, $($tail:ty),+) => {
        impl_from!($t);
        impl_from!($($tail),+);
    };

    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_from!($($tail),+);
    };
}

primitives!(; types, impl_from);
