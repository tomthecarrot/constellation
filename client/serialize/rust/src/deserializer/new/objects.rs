use bimap::BiHashMap;

use crate::{rs, types::ObjectsIdx};

#[derive(Default)]
pub struct InstantiatedObjects(BiHashMap<rs::ObjectHandle, ObjectsIdx>);
impl InstantiatedObjects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, handle: rs::ObjectHandle, idx: ObjectsIdx) {
        assert!(matches!(
            self.0.insert(handle, idx),
            bimap::Overwritten::Neither
        ))
    }

    pub fn get_idx(&self, handle: rs::ObjectHandle) -> ObjectsIdx {
        self.0
            .get_by_left(&handle)
            .copied()
            .expect("Tried to get a handle that didnt exist")
    }

    pub fn get_handle(&self, idx: ObjectsIdx) -> rs::ObjectHandle {
        self.0
            .get_by_right(&idx)
            .copied()
            .expect("Tried to get an index that didn't exist")
    }
}
