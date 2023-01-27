use bimap::BiHashMap;
use eyre::{eyre, Result};

use std::collections::HashMap;

use crate::rs;
use crate::types::{ObjectsIdx, StatesIdx};

/// Tracks the relationships between indicies in the flatbuffer and instantiated states
/// in the baseline. This assumes that all `State<ObjectHandle>`s in the baseline are
/// supposed to point to the null object at this stage in deserialization.
#[derive(Default)]
pub struct InstantiatedStates {
    /// All instantiated states that reference objects. These will be storing the null
    /// object handle initially.
    states: BiHashMap<StatesIdx, rs::StateHandle<rs::ObjectHandle>>,
    /// A mapping of null states to the objects in the flatbuffer they are supposed
    /// to reference.
    obj_refs: HashMap<StatesIdx, ObjectsIdx>,
}
impl InstantiatedStates {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tracks which index in the flatbuffer corresponds to which deserialized state.
    ///
    /// # Panics
    /// Panics if the added state never had its obj refs tracked.
    pub fn track_instantiated_state(
        &mut self,
        idx: StatesIdx,
        handle: rs::StateHandle<rs::ObjectHandle>,
    ) -> Result<()> {
        self.states
            .insert_no_overwrite(idx, handle)
            .map_err(|_| eyre!("State was already tracked!"))
    }

    /// Tracks which states reference which objects in the flatbuffer. Must call this
    /// before `track_instantiated_state`.
    pub fn track_obj_reference(
        &mut self,
        state_idx: StatesIdx,
        referenced_obj_idx: ObjectsIdx,
    ) -> Result<()> {
        if self
            .obj_refs
            .insert(state_idx, referenced_obj_idx)
            .is_none()
        {
            Err(eyre!("object reference was already tracked!"))
        } else {
            Ok(())
        }
    }

    pub fn get_state_handle(&self, idx: StatesIdx) -> rs::StateHandle<rs::ObjectHandle> {
        self.states
            .get_by_left(&idx)
            .copied()
            .expect("state idx was never added")
    }

    pub fn get_state_idx(&self, handle: rs::StateHandle<rs::ObjectHandle>) -> StatesIdx {
        self.states
            .get_by_right(&handle)
            .copied()
            .expect("Only added state handles should have been used.")
    }

    pub fn get_obj_ref_idx(&self, state_idx: StatesIdx) -> ObjectsIdx {
        self.obj_refs
            .get(&state_idx)
            .copied()
            .expect("State was never tracked!")
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (StatesIdx, rs::StateHandle<rs::ObjectHandle>, ObjectsIdx)> + '_ {
        self.states
            .iter()
            .map(|(s_idx, h)| (*s_idx, *h, self.get_obj_ref_idx(*s_idx)))
    }
}
