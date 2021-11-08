use crate::contract::properties::{StateHandle, TPData};

pub struct Object {
    // TODO: Type erase the T in StateHandle<T>?
// states: Vec<StateHandle<impl TPData>>, // map from StateID -> StateHandle
}

pub type ObjectHandle = arena::Index<Object>;
