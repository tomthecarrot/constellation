use crate::properties::data::TPData;

use std::marker::PhantomData;
use typemap::TypeMap;

// TODO: figure out data in a channel
pub struct Channel<T: TPData>(T);

pub type ChannelID<T> = arena::Index<Channel<T>>;

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub struct ChannelArenaID<T>(PhantomData<T>);
impl<T: 'static + TPData> typemap::Key for ChannelArenaID<T> {
    type Value = arena::Arena<Channel<T>>;
}

pub type ChannelArenaMap = TypeMap<arena::Arena<Channel<dyn TPData>>>;
