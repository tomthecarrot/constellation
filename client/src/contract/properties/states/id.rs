use crate::contract::properties::dynamic::__macro::{DynEnum, DynTpPropId};
use crate::contract::properties::traits::ITpProperty;
use crate::contract::ContractDataHandle;

use std::marker::PhantomData;

/// Represents a particular state field of a contract. For actual state data of
/// a specific object, see [`StateHandle`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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

DynTpPropId!(DynStateId, StateId);
