use bimap::BiHashMap;
use eyre::{eyre, Result};

use crate::{rs, types::ObjectsIdx};

#[derive(Default)]
pub struct InstantiatedObjects(BiHashMap<rs::ObjectHandle, ObjectsIdx>);
impl InstantiatedObjects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, handle: rs::ObjectHandle, idx: ObjectsIdx) {
        assert!(
            matches!(self.0.insert(handle, idx), bimap::Overwritten::Neither),
            "the object was already added!"
        )
    }

    pub fn get_idx(&self, handle: rs::ObjectHandle) -> Result<ObjectsIdx> {
        self.0
            .get_by_left(&handle)
            .copied()
            .ok_or_else(|| eyre!("Tried to access an object that didnt exist"))
    }

    pub fn get_handle(&self, idx: ObjectsIdx) -> Result<rs::ObjectHandle> {
        self.0
            .get_by_right(&idx)
            .copied()
            .ok_or_else(|| eyre!("Tried to access an object that didn't exist"))
    }
}
