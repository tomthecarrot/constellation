use crate::contract::properties::data::TPData;
use crate::contract::ContractHandle;

use std::marker::PhantomData;
use typemap::TypeMap;

pub type StateHandle<T> = arena::Index<State<T>>;

pub struct State<T: TPData>(T);

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct StateArenaID<T>(PhantomData<T>);
impl<T: 'static + TPData> typemap::Key for StateArenaID<T> {
    type Value = arena::Arena<State<T>>;
}

pub type StateArenaMap = TypeMap;

/// Represents a particular state field of a contract. For actual state data of
/// a specific object, see [`StateHandle`].
#[derive(Copy, Clone, Hash, Debug)]
pub struct StateID {
    idx: usize, // idx into an object's state properties
    contract: ContractHandle,
}
