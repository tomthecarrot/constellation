use super::State;
use crate::baseline::Baseline;
use crate::contract::properties::dynamic::TpPropertyType;
use crate::contract::properties::traits::ITpPropertyStatic;

use eyre::{eyre, Result};

/// Any type that can be used as a handle for a `State<T>` (or a `DynState`).
///
/// If static typing is strictly necessary, use `StateHandle<T>` directly
pub trait IStateHandle {
    type OutputRef<'a>;
    type OutputMut<'a>;

    fn get<'a>(&self, baseline: &'a Baseline) -> Result<Self::OutputRef<'a>>;
    fn get_mut<'a>(&self, baseline: &'a mut Baseline) -> Result<Self::OutputMut<'a>>;

    fn prop_type(&self) -> TpPropertyType;
}

pub type StateHandle<T> = arena::Index<State<T>>;
impl<T: ITpPropertyStatic> IStateHandle for StateHandle<T> {
    type OutputRef<'a> = &'a State<T>;
    type OutputMut<'a> = &'a mut State<T>;

    fn get<'a>(&self, baseline: &'a Baseline) -> Result<Self::OutputRef<'a>> {
        let arena = baseline
            .states
            .get()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(*self)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn get_mut<'a>(&self, baseline: &'a mut Baseline) -> Result<Self::OutputMut<'a>> {
        let arena = baseline
            .states
            .get_mut()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(*self)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn prop_type(&self) -> TpPropertyType {
        T::PROPERTY_TYPE
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]
    use super::*;

    use crate::contract::properties::primitives;
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;

    use derive_more::{From, Into};
    use ref_cast::RefCast;
    use rsharp::remangle;
    use safer_ffi::prelude::*;

    macro_rules! monomorphize {
        // Base case
        ($path:literal, $t:ty $(,)?) => {
            paste::paste! {
                mod [<_StateHandle_ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, Into, RefCast, Copy, Clone, Eq, PartialEq)]
                    #[repr(C)]
                    pub struct [<StateHandle_ $t:camel>] {
                        pub inner: StateHandle<$t>,
                    }
                    pub use [<StateHandle_ $t:camel>] as Monomorphized;

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<StateHandle_ $t:camel __drop>](h: repr_c::Box<Monomorphized>) {
                        drop(h)
                    }
                }
                pub use [<_StateHandle_ $t:camel>]::[<StateHandle_ $t:camel>];

             }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }
    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    primitives!(; types, monomorphize, "tp_client::contract::properties::states");
}
