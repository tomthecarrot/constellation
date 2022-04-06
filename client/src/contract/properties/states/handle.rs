use super::State;
use crate::baseline::Baseline;
use crate::contract::properties::dynamic::TpPropertyType;
use crate::contract::properties::traits::ITpPropertyStatic;

use eyre::{eyre, Result};

/// Any type that can be used as a handle for a `State<T>` (or a `DynState`).
///
/// If static typing is strictly necessary, use `StateHandle<T>` directly
pub trait IStateHandle {
    type OutputRef<'a>;
    type OutputMut<'a>;

    fn get<'a>(&self, baseline: &'a Baseline) -> Result<Self::OutputRef<'a>>;
    fn get_mut<'a>(&self, baseline: &'a mut Baseline) -> Result<Self::OutputMut<'a>>;

    fn prop_type(&self) -> TpPropertyType;
}

pub type StateHandle<T> = arena::Index<State<T>>;
impl<T: ITpPropertyStatic> IStateHandle for StateHandle<T> {
    type OutputRef<'a> = &'a State<T>;
    type OutputMut<'a> = &'a mut State<T>;

    fn get<'a>(&self, baseline: &'a Baseline) -> Result<Self::OutputRef<'a>> {
        let arena = baseline
            .states
            .get()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(*self)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn get_mut<'a>(&self, baseline: &'a mut Baseline) -> Result<Self::OutputMut<'a>> {
        let arena = baseline
            .states
            .get_mut()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(*self)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn prop_type(&self) -> TpPropertyType {
        T::PROPERTY_TYPE
    }
}
