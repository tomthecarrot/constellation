mod channel;
pub mod dyn_channel;
pub mod dyn_handle;
mod handle;

mod misc;

pub use self::channel::{Channel, Keyframe};
pub use self::dyn_channel::{apply_to_channel, apply_to_channel_mut, apply_to_channel_ref};
pub use self::dyn_channel::{DynChannel, DynChannelMut, DynChannelRef};
pub use self::dyn_handle::DynChannelHandle;
pub use self::handle::{ChannelHandle, IChannelHandle};
pub use crate::contract::properties::dynamic::apply_to_channel_id;

use crate::contract::properties::dynamic::TpPropertyType;
use crate::contract::properties::dynamic::__macro::{DynEnum, DynTpPropId};
use crate::contract::properties::traits::{ITpProperty, ITpPropertyStatic};
use crate::contract::ContractDataHandle;

use std::any::TypeId;
use std::marker::PhantomData;
use typemap::ShareMap;

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub(crate) struct ChannelArenaHandle<T: ITpPropertyStatic>(PhantomData<T>);
impl<T: ITpPropertyStatic> typemap::Key for ChannelArenaHandle<T> {
    type Value = arena::Arena<Channel<T>>;
}

pub struct ChannelArenaMap(pub ShareMap);
impl ChannelArenaMap {
    pub fn new() -> Self {
        Self(ShareMap::custom())
    }

    pub fn get<T: ITpPropertyStatic>(&self) -> Option<&arena::Arena<Channel<T>>> {
        self.0.get::<ChannelArenaHandle<T>>()
    }

    pub fn get_mut<T: ITpPropertyStatic>(&mut self) -> Option<&mut arena::Arena<Channel<T>>> {
        self.0.get_mut::<ChannelArenaHandle<T>>()
    }
}

/// Represents a particular channel field of a contract. For actual channel data
/// of a specific object, see [`ChannelHandle`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct ChannelId<T: ITpProperty> {
    idx: usize, // idx into an object's channel properties
    contract: ContractDataHandle,
    _phantom: PhantomData<T>,
}
impl<T: ITpProperty> ChannelId<T> {
    pub fn contract(&self) -> ContractDataHandle {
        self.contract
    }

    pub(crate) fn idx(&self) -> usize {
        self.idx
    }

    /// This is only exposed so that contract macros can call it. It isn't intended
    /// for direct use by API clients
    #[doc(hidden)]
    pub fn new(idx: usize, contract: ContractDataHandle) -> Self {
        Self {
            idx,
            contract,
            _phantom: PhantomData,
        }
    }
}

pub trait IChannels {
    fn type_ids() -> &'static [TypeId];
    fn enumerate_types() -> &'static [TpPropertyType];
}

impl IChannels for () {
    fn type_ids() -> &'static [TypeId] {
        &[]
    }

    fn enumerate_types() -> &'static [TpPropertyType] {
        &[]
    }
}

DynTpPropId!(DynChannelId, ChannelId);

super::prop_iter!(ChannelsIter, IChannels, DynChannelId);
