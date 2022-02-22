use derive_more::From;

use crate::contract::properties::dynamic::{DynTpProperty, DynTpPropertyMut, DynTpPropertyRef};

/// Holds all information related to a state with a dynamic type.
pub struct DynState(pub DynTpProperty);

#[derive(From, Debug, Clone, PartialEq)]
pub struct DynStateRef<'a>(pub DynTpPropertyRef<'a>);

#[derive(From, Debug, PartialEq)]
pub struct DynStateMut<'a>(pub DynTpPropertyMut<'a>);
