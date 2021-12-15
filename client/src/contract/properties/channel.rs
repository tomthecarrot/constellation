use crate::contract::properties::data::ITpProperty;
use crate::contract::ContractId;

use std::marker::PhantomData;
use typemap::ShareMap;

// TODO: figure out data in a channel
pub struct Channel<T: ITpProperty>(pub T);

pub type ChannelHandle<T> = arena::Index<Channel<T>>;

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct ChannelArenaHandle<T: ITpProperty>(PhantomData<T>);
impl<T: ITpProperty> typemap::Key for ChannelArenaHandle<T> {
    type Value = arena::Arena<Channel<T>>;
}

pub type ChannelArenaMap = ShareMap;

/// Represents a particular channel field of a contract. For actual channel data
/// of a specific object, see [`ChannelHandle`].
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ChannelId<T: ITpProperty> {
    idx: usize, // idx into an object's channel properties
    contract: ContractId,
    _phantom: PhantomData<T>,
}
impl<T: ITpProperty> ChannelId<T> {
    pub fn contract(&self) -> ContractId {
        self.contract
    }

    pub(crate) fn idx(&self) -> usize {
        self.idx
    }
}
