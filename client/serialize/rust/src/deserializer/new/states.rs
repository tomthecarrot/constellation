use bimap::BiHashMap;
use std::collections::HashMap;

use crate::rs;
use crate::types::{ObjectsIdx, StatesIdx};

/// Tracks the relationships between indicies in the flatbuffer and instantiated states
/// in the baseline. This assumes that all `State<ObjectHandle>`s in the baseline are
/// supposed to point to the null object at this stage in deserialization.
#[derive(Default)]
pub struct InstantiatedStates {
    /// All instantiated states, including ones that reference null objects.
    pub instantiations: BiHashMap<StatesIdx, rs::DynStateHandle>,
    /// A mapping of null states to the objects in the flatbuffer they are supposed
    /// to reference.
    pub null_map: HashMap<rs::StateHandle<rs::ObjectHandle>, ObjectsIdx>,
}
impl InstantiatedStates {
    pub fn new() -> Self {
        Self::default()
    }

    /// `None` if the contract wasn't instantiated yet.
    pub fn get_handle(&self, idx: StatesIdx) -> Option<rs::DynStateHandle> {
        self.instantiations.get_by_left(&idx).copied()
    }

    pub fn get_idx(&self, handle: rs::DynStateHandle) -> StatesIdx {
        self.instantiations
            .get_by_right(&handle)
            .copied()
            .expect("Only instantiated state handles should have been used. Howmst?")
    }

    pub fn is_null_idx(&self, idx: StatesIdx) -> bool {
        let h = self
            .get_handle(idx)
            .expect("idx has no corresponding handle.");
        self.is_null_handle(h)
    }

    pub fn is_null_handle(&self, handle: rs::DynStateHandle) -> bool {
        use tp_client::contract::properties::states::dyn_handle::DynStateHandlePrimitive as P;
        use tp_client::contract::properties::states::DynStateHandle;
        let h = match handle {
            DynStateHandle::Primitive(p) => match p {
                P::ObjectHandle(h) => h,
                _ => return false,
            },
            DynStateHandle::Vec(_) => panic!("unsupported"),
        };
        assert!(
            self.null_map.contains_key(&h),
            "this is a bug: All object handles should be null"
        );
        return true;
    }

    pub fn get_original_idx(&self, handle: rs::StateHandle<rs::ObjectHandle>) -> ObjectsIdx {
        self.null_map
            .get(&handle)
            .copied()
            .expect("`handle` should have been present in the list of null states, but wasnt")
    }
}
