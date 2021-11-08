use crate::properties::{StateID, TPData};

pub struct Object {
    // TODO: Type erase the T in StateID<T>?
// states: Vec<StateID<impl TPData>>, // map from StateName -> StateID
}

pub type ObjectID = arena::Index<Object>;
