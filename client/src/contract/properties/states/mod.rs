pub mod dyn_handle;
pub mod dyn_state;
pub mod handle;
pub mod id;

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
pub struct State<T: ITpPropertyStatic>(pub T);

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
