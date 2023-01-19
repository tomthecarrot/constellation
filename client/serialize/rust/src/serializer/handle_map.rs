use bimap::BiHashMap;
use std::ops::Index;

use crate::types::{ContractsIdx, ObjectsIdx, StatesIdx};
use crate::{fb, rs};

#[derive(Default, Debug)]
pub struct HandleMap {
    /// Handles to objects
    pub objects: BiHashMap<rs::ObjectHandle, ObjectsIdx>,
    /// Handles to contracts
    pub contracts: BiHashMap<rs::ContractDataHandle, ContractsIdx>,
    /// Handles to State<ContractDataHandle>
    pub contract_states: BiHashMap<rs::StateHandle<rs::ContractDataHandle>, StatesIdx>,
    /// Handles to State<ObjectHandle>
    pub object_states: BiHashMap<rs::StateHandle<rs::ObjectHandle>, StatesIdx>,
}
impl HandleMap {
    pub fn insert_object(&mut self, handle: rs::ObjectHandle, idx: ObjectsIdx) {
        self.objects.insert(handle, idx);
    }

    pub fn insert_contract(&mut self, handle: rs::ContractDataHandle, idx: ContractsIdx) {
        self.contracts.insert(handle, idx);
    }

    pub fn insert_contract_state(
        &mut self,
        handle: rs::StateHandle<rs::ContractDataHandle>,
        idx: StatesIdx,
    ) {
        self.contract_states.insert(handle, idx);
    }

    pub fn insert_object_state(
        &mut self,
        handle: rs::StateHandle<rs::ObjectHandle>,
        idx: StatesIdx,
    ) {
        self.object_states.insert(handle, idx);
    }
}

impl Index<rs::ObjectHandle> for HandleMap {
    type Output = ObjectsIdx;
    fn index(&self, index: rs::ObjectHandle) -> &Self::Output {
        self.objects.get_by_left(&index).expect("No such handle")
    }
}
impl Index<rs::ContractDataHandle> for HandleMap {
    type Output = ContractsIdx;
    fn index(&self, index: rs::ContractDataHandle) -> &Self::Output {
        self.contracts.get_by_left(&index).expect("No such handle")
    }
}
impl Index<ObjectsIdx> for HandleMap {
    type Output = rs::ObjectHandle;
    fn index(&self, index: ObjectsIdx) -> &Self::Output {
        self.objects.get_by_right(&index).expect("No such handle")
    }
}
impl Index<ContractsIdx> for HandleMap {
    type Output = rs::ContractDataHandle;
    fn index(&self, index: ContractsIdx) -> &Self::Output {
        self.contracts.get_by_right(&index).expect("No such handle")
    }
}
