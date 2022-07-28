pub mod dyn_handle;
pub mod dyn_state;
mod handle;
pub(crate) mod id;

mod iter;

pub use self::dyn_handle::DynStateHandle;
pub use self::dyn_state::DynState;
pub use self::handle::{IStateHandle, StateHandle};
pub use self::id::{DynStateId, StateId};
pub use self::iter::{IStates, StatesIter};
pub use crate::contract::properties::dynamic::apply_to_state_id;

use crate::contract::properties::traits::ITpPropertyStatic;

use std::marker::PhantomData;
use typemap::ShareMap;

/// Holds all information related to a state with a statically-known type `T`.
#[derive(Debug, PartialEq)]
pub struct State<T: ITpPropertyStatic> {
    pub value: T,
}

impl<T: ITpPropertyStatic> State<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub(crate) struct StateArenaHandle<T: ITpPropertyStatic>(PhantomData<T>);
impl<T: ITpPropertyStatic> typemap::Key for StateArenaHandle<T> {
    type Value = arena::Arena<State<T>>;
}

pub struct StateArenaMap(pub ShareMap);
impl StateArenaMap {
    pub fn new() -> Self {
        Self(ShareMap::custom())
    }

    pub fn get<T: ITpPropertyStatic>(&self) -> Option<&arena::Arena<State<T>>> {
        self.0.get::<StateArenaHandle<T>>()
    }

    pub fn get_mut<T: ITpPropertyStatic>(&mut self) -> Option<&mut arena::Arena<State<T>>> {
        self.0.get_mut::<StateArenaHandle<T>>()
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]

    pub use super::handle::c_api::*;
    pub use super::id::c_api::*;

    use super::*;
    use crate::contract::properties::c_api::{c_types, impl_from_refcast};
    use crate::contract::properties::primitives;
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;

    use derive_more::{From, Into};
    use ref_cast::RefCast;
    use rsharp::remangle;
    use safer_ffi::prelude::*;

    macro_rules! monomorphize {
        ($path:literal, $t:ty $(,)?) => {
            paste::paste! {
                // Module is simply to prevent name collisions on the rust side. It does
                // nothing for C
                mod [<_State_ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, Into, RefCast)]
                    #[repr(C)]
                    pub struct [<State_ $t:camel>] {
                        pub inner: State<$t>
                    }

                    pub use [<State_ $t:camel>] as Monomorphized;
                    impl_from_refcast!(State<$t>, Monomorphized);

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<State_ $t:camel __new>](value: repr_c::Box<c_types::$t>) -> repr_c::Box<Monomorphized> {
                        let value = $t::from(*value);
                        repr_c::Box::new(State::new(value).into())
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<State_ $t:camel __drop>](s: repr_c::Box<Monomorphized>) {
                        drop(s)
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<State_ $t:camel __value>]<'a>(state: &'a Monomorphized) -> &'a c_types::$t {
                        (&state.inner.value).into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<State_ $t:camel __value_set>]<'a>(state: &'a mut Monomorphized, new_value: ::safer_ffi::boxed::Box<c_types::$t>) {
                        state.inner.value = (*new_value).into();
                    }
                }
                pub use [<_State_ $t:camel>]::Monomorphized as [<State_ $t:camel>];
            }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }

    // This is like doing `monomorphize!("whatever", State, u8, u16, ...)
    primitives!(; types, monomorphize, "tp_client::contract::properties::states");
}
