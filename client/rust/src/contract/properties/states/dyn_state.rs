use derive_more::{From, Into};

use crate::contract::properties::dynamic::{DynTpProperty, DynTpPropertyMut, DynTpPropertyRef};
use crate::contract::properties::primitives;
use crate::contract::properties::states::State;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

/// Holds all information related to a state with a dynamic type.
#[derive(From, Into, Debug, Clone, PartialEq)]
pub struct DynState(pub DynTpProperty);

#[derive(From, Into, Debug, Clone, PartialEq)]
pub struct DynStateRef<'a>(pub DynTpPropertyRef<'a>);

#[derive(From, Into, Debug, PartialEq)]
pub struct DynStateMut<'a>(pub DynTpPropertyMut<'a>);

macro_rules! impl_from {
    // base case
    ($t:ty) => {
        impl From<State<$t>> for DynState {
            fn from(other: State<$t>) -> Self {
                DynTpProperty::from(other.value).into()
            }
        }

        impl From<State<Vec<$t>>> for DynState {
            fn from(other: State<Vec<$t>>) -> Self {
                let dyn_prop = DynTpProperty::from(other.value);
                Self(dyn_prop)
            }
        }

        impl <'a> From<&'a State<$t>> for DynStateRef<'a> {
            fn from(other: &'a State<$t>) -> Self {
                let dyn_prop = DynTpPropertyRef::from(&other.value);
                Self(dyn_prop)
            }
        }

        impl<'a> From<&'a State<Vec<$t>>> for DynStateRef<'a> {
            fn from(other: &'a State<Vec<$t>>) -> Self {
                let dyn_prop = DynTpPropertyRef::from(&other.value);
                Self(dyn_prop)
            }
        }

        impl <'a> From<&'a mut State<$t>> for DynStateMut<'a> {
            fn from(other: &'a mut State<$t>) -> Self {
                let dyn_prop = DynTpPropertyMut::from(&mut other.value);
                Self(dyn_prop)
            }
        }

        impl<'a> From<&'a mut State<Vec<$t>>> for DynStateMut<'a> {
            fn from(other: &'a mut State<Vec<$t>>) -> Self {
                let dyn_prop = DynTpPropertyMut::from(&mut other.value);
                Self(dyn_prop)
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
