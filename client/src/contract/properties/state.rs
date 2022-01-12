use crate::baseline::Baseline;
use crate::contract::properties::data::ITpProperty;
use crate::contract::ContractDataHandle;

use std::any::TypeId;
use std::marker::PhantomData;
use typemap::ShareMap;

pub type StateHandle<T> = arena::Index<State<T>>;

pub struct State<T: ITpProperty>(pub T);

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct StateArenaHandle<T: ITpProperty>(PhantomData<T>);
impl<T: ITpProperty> typemap::Key for StateArenaHandle<T> {
    type Value = arena::Arena<State<T>>;
}

pub type StateArenaMap = ShareMap;

/// Represents a particular state field of a contract. For actual state data of
/// a specific object, see [`StateHandle`].
#[derive(Copy, Clone, Hash, Debug)]
pub struct StateId<T: ITpProperty> {
    idx: usize, // idx into an object's state properties
    contract: ContractDataHandle,
    _phantom: PhantomData<T>,
}
impl<T: ITpProperty> StateId<T> {
    pub fn contract(&self) -> ContractDataHandle {
        self.contract
    }

    pub(crate) fn idx(&self) -> usize {
        self.idx
    }

    pub fn new(idx: usize, contract: ContractDataHandle) -> Self {
        Self {
            idx,
            contract,
            _phantom: PhantomData,
        }
    }
}

pub trait IStates {
    fn type_ids() -> &'static [TypeId];
}

impl IStates for () {
    fn type_ids() -> &'static [TypeId] {
        &[]
    }
}
