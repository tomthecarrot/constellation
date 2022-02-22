use derive_more::From;

use crate::contract::properties::dynamic::{DynTpProperty, DynTpPropertyMut, DynTpPropertyRef};

/// Holds all information related to a state with a dynamic type.
pub struct DynChannel(pub DynTpProperty);

#[derive(From, Debug, Clone, PartialEq)]
pub struct DynChannelRef<'a>(pub DynTpPropertyRef<'a>);

#[derive(From, Debug, PartialEq)]
pub struct DynChannelMut<'a>(pub DynTpPropertyMut<'a>);
