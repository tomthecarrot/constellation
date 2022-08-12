use super::DynStateId;
use crate::contract::properties::dynamic::TpPropertyType;
use crate::contract::properties::prop_iter;

use std::any::TypeId;

pub trait IStates: Sized {
    fn type_ids() -> &'static [TypeId];
    fn enumerate_types() -> &'static [TpPropertyType];
    fn field_names() -> &'static [&'static str];
}

impl IStates for () {
    fn type_ids() -> &'static [TypeId] {
        &[]
    }

    fn enumerate_types() -> &'static [TpPropertyType] {
        &[]
    }

    fn field_names() -> &'static [&'static str] {
        &[]
    }
}

prop_iter!(StatesIter, IStates, DynStateId);
