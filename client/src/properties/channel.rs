use crate::{contract::ContractID, properties::data::TPData};

use std::marker::PhantomData;
use typemap::TypeMap;

// TODO: figure out data in a channel
pub struct Channel<T: TPData>(T);

pub type ChannelID<T> = arena::Index<Channel<T>>;

/// Represents a particular channel field of a contract. For actual channel data
/// of a specific object, see [`ChannelID`].
pub struct ChannelName {
    idx: usize,
    contract: ContractID,
}

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct ChannelArenaID<T>(PhantomData<T>);
impl<T: 'static + TPData> typemap::Key for ChannelArenaID<T> {
    type Value = arena::Arena<Channel<T>>;
}

pub type ChannelArenaMap = TypeMap;
