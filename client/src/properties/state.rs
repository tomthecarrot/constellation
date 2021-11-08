use crate::{contract::ContractID, properties::data::TPData};

use std::marker::PhantomData;
use typemap::TypeMap;

pub type StateID<T> = arena::Index<State<T>>;

/// Represents a particular state field of a contract. For actual state data of
/// a specific object, see [`StateID`].
#[derive(Copy, Clone, Hash, Debug)]
pub struct StateName {
    idx: usize, // idx into an object's state properties
    contract: ContractID,
}

pub struct State<T: TPData>(T);

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct StateArenaID<T>(PhantomData<T>);
impl<T: 'static + TPData> typemap::Key for StateArenaID<T> {
    type Value = arena::Arena<State<T>>;
}

pub type StateArenaMap = TypeMap;
