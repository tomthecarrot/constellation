use bimap::BiHashMap;

use crate::rs;
use crate::types::ContractsIdx;

/// Tracks the relationships between indicies in the flatbuffer and instantiated
/// contracts in the baseline.
#[derive(Debug, Default)]
pub struct InstantiatedContracts(BiHashMap<ContractsIdx, rs::ContractDataHandle>);
impl InstantiatedContracts {
    pub fn new() -> Self {
        Self::default()
    }

    /// None if the contract wasn't instantiated yet.
    pub fn get_handle(&self, idx: ContractsIdx) -> Option<rs::ContractDataHandle> {
        self.0.get_by_left(&idx).copied()
    }

    pub fn get_idx(&self, handle: rs::ContractDataHandle) -> ContractsIdx {
        self.0
            .get_by_right(&handle)
            .copied()
            .expect("Only instantiated contract handles should have been used. Howmst")
    }

    pub fn register_contract(&mut self, idx: ContractsIdx, handle: rs::ContractDataHandle) {
        assert!(matches!(
            self.0.insert(idx, handle),
            bimap::Overwritten::Neither
        ));
    }
}
