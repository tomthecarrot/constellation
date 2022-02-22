use super::DynStateId;
use crate::contract::properties::dynamic::TpPropertyType;
use crate::contract::properties::prop_iter;

use std::any::TypeId;

pub trait IStates: Sized {
    fn type_ids() -> &'static [TypeId];
    fn enumerate_types() -> &'static [TpPropertyType];
}

impl IStates for () {
    fn type_ids() -> &'static [TypeId] {
        &[]
    }

    fn enumerate_types() -> &'static [TpPropertyType] {
        &[]
    }
}

prop_iter!(StatesIter, IStates, DynStateId);
