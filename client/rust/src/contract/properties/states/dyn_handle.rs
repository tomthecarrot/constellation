use super::dyn_state::{DynStateMut, DynStateRef};
use super::{handle::StateHandle, IStateHandle};
use crate::apply_to_state_handle;
use crate::contract::properties::dynamic::__macro::DynEnum;
use crate::contract::properties::dynamic::{
    DynTpPropertyMut, DynTpPropertyRef, TpPrimitiveType, TpPropertyType,
};
use crate::contract::properties::primitives;
use crate::contract::properties::traits::ITpPropertyStatic;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

DynEnum!(DynStateHandle, StateHandle | derive(Clone, PartialEq));
impl IStateHandle for DynStateHandle {
    type OutputRef<'a> = DynStateRef<'a>;
    type OutputMut<'a> = DynStateMut<'a>;

    fn get<'a>(
        &self,
        baseline: &'a crate::baseline::Baseline,
    ) -> eyre::Result<Self::OutputRef<'a>> {
        // Matches on the type and then calls the appropriate generic function in baseline
        apply_to_state_handle!(*self, |h: StateHandle<_>| {
            let state = h.get(baseline)?;
            let prop_ref: DynTpPropertyRef<'a> = DynTpPropertyRef::from(&state.value);
            Ok(prop_ref.into())
        })
    }

    fn get_mut<'a>(
        &self,
        baseline: &'a mut crate::baseline::Baseline,
    ) -> eyre::Result<Self::OutputMut<'a>> {
        fn make_dyn<'a, T>(
            h: StateHandle<T>,
            baseline: &'a mut crate::baseline::Baseline,
        ) -> eyre::Result<DynStateMut<'a>>
        where
            T: ITpPropertyStatic,
            &'a mut T: Into<DynTpPropertyMut<'a>>,
        {
            let state = h.get_mut(baseline)?;
            let prop_mut: DynTpPropertyMut<'a> = (&mut state.value).into();
            Ok(prop_mut.into())
        }

        // Not sure why this can't just use `apply_to_state_handle!()`, but it
        // can't figure out the lifetime inference with that approach. So we match
        // with an enum + macro instead.
        macro_rules! match_helper {
            ($enum_ident:ident, $enum_type:ident, $($variant:ident),+ $(,)?) => {
                match $enum_ident {
                    $($enum_type::$variant(h) => make_dyn(h, baseline)),*
                }
            }
        }

        match *self {
            DynStateHandle::Primitive(p) => {
                primitives!(idents, match_helper, p, DynStateHandlePrimitive)
            }
            DynStateHandle::Vec(v) => primitives!(idents, match_helper, v, DynStateHandleVec),
        }
    }

    fn prop_type(&self) -> TpPropertyType {
        use DynStateHandlePrimitive as P;
        use DynStateHandleVec as V;

        // Maps enum to `TpPrimitiveType` by expanding to match on variants
        macro_rules! helper_match {
            ($enum:ident, $enum_type:ident, $($ident:ident),+ $(,)?) => {
                match $enum {
                    $(
                        $enum_type::$ident(_) => TpPrimitiveType::$ident,
                    )*
                }
            };
        }

        match self {
            Self::Primitive(p) => {
                TpPropertyType::Primitive(primitives!(idents, helper_match, p, P))
            }
            Self::Vec(v) => TpPropertyType::Vec(primitives!(idents, helper_match, v, V)),
        }
    }
}

impl Copy for DynStateHandlePrimitive {}
impl Copy for DynStateHandleVec {}
impl Copy for DynStateHandle {}

macro_rules! impl_from {
    // base case
    ($t:ty) => {
        impl From<StateHandle<$t>> for DynStateHandle {
            fn from(other: StateHandle<$t>) -> Self {
                Self::Primitive(DynStateHandlePrimitive::from(other))
            }
        }

        impl From<StateHandle<Vec<$t>>> for DynStateHandle {
            fn from(other: StateHandle<Vec<$t>>) -> Self {
                Self::Vec(DynStateHandleVec::from(other))
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
