use crate::contract::properties::{ChannelHandle, State, StateHandle, TPData};

use derive_more::From;

pub enum StateAction<T: TPData> {
    Write(StateHandle<T>, T),
    Assert(StateHandle<T>, T),
}

pub enum ChannelAction<T: TPData> {
    Write(ChannelHandle<T>, T),
    Read(ChannelHandle<T>, T),
}

#[derive(From)]
pub enum PropertyAction<T: TPData> {
    State(StateAction<T>),
    Channel(ChannelAction<T>),
}

#[derive(From)]
pub enum Action<T: TPData = Box<dyn TPData>> {
    Property(PropertyAction<T>),
}

pub struct Collaction {
    actions: Vec<Action>,
}

pub struct CollactionResult {
    collaction: Collaction,
    was_accepted: bool,
}
