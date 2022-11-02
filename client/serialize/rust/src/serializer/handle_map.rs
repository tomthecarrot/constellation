use bimap::BiHashMap;
use std::ops::Index;

use crate::{c, t};

/// Index into `objects` vec
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ObjectsIdx(pub usize);

/// Index into `contracts` vec
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ContractsIdx(pub usize);

/// Index into `states` vec
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct StatesIdx(pub usize);

#[derive(Default, Debug)]
pub struct HandleMap {
    /// Handles to objects
    pub objects: BiHashMap<c::ObjectHandle, ObjectsIdx>,
    /// Handles to contracts
    pub contracts: BiHashMap<c::ContractDataHandle, ContractsIdx>,
    /// Handles to State<ContractDataHandle>
    pub contract_states: BiHashMap<c::StateHandle<c::ContractDataHandle>, StatesIdx>,
    /// Handles to State<ObjectHandle>
    pub object_states: BiHashMap<c::StateHandle<c::ObjectHandle>, StatesIdx>,
}
impl HandleMap {
    pub fn insert_object(&mut self, handle: c::ObjectHandle, idx: ObjectsIdx) {
        self.objects.insert(handle, idx);
    }

    pub fn insert_contract(&mut self, handle: c::ContractDataHandle, idx: ContractsIdx) {
        self.contracts.insert(handle, idx);
    }

    pub fn insert_contract_state(
        &mut self,
        handle: c::StateHandle<c::ContractDataHandle>,
        idx: StatesIdx,
    ) {
        self.contract_states.insert(handle, idx);
    }

    pub fn insert_object_state(&mut self, handle: c::StateHandle<c::ObjectHandle>, idx: StatesIdx) {
        self.object_states.insert(handle, idx);
    }
}

impl Index<c::ObjectHandle> for HandleMap {
    type Output = ObjectsIdx;
    fn index(&self, index: c::ObjectHandle) -> &Self::Output {
        self.objects.get_by_left(&index).expect("No such handle")
    }
}
impl Index<c::ContractDataHandle> for HandleMap {
    type Output = ContractsIdx;
    fn index(&self, index: c::ContractDataHandle) -> &Self::Output {
        self.contracts.get_by_left(&index).expect("No such handle")
    }
}
impl Index<ObjectsIdx> for HandleMap {
    type Output = c::ObjectHandle;
    fn index(&self, index: ObjectsIdx) -> &Self::Output {
        self.objects.get_by_right(&index).expect("No such handle")
    }
}
impl Index<ContractsIdx> for HandleMap {
    type Output = c::ContractDataHandle;
    fn index(&self, index: ContractsIdx) -> &Self::Output {
        self.contracts.get_by_right(&index).expect("No such handle")
    }
}
