use crate::contract::properties::data::TPData;
use crate::contract::ContractHandle;

use std::marker::PhantomData;
use typemap::ShareMap;

// TODO: figure out data in a channel
pub struct Channel<T: TPData>(T);

pub type ChannelHandle<T> = arena::Index<Channel<T>>;

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct ChannelArenaHandle<T>(PhantomData<T>);
impl<T: 'static + TPData> typemap::Key for ChannelArenaHandle<T> {
    type Value = arena::Arena<Channel<T>>;
}

pub type ChannelArenaMap = ShareMap;

/// Represents a particular channel field of a contract. For actual channel data
/// of a specific object, see [`ChannelHandle`].
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ChannelID {
    idx: usize, // idx into an object's channel properties
    contract: ContractHandle,
}
impl ChannelID {
    pub fn contract(&self) -> ContractHandle {
        self.contract
    }

    pub(crate) fn idx(&self) -> usize {
        self.idx
    }
}
