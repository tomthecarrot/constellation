use crate::contract::properties::{ChannelHandle, State, StateHandle, TPData};

pub enum StateAction<T: TPData> {
    Write(StateHandle<T>, T),
    Assert(StateHandle<T>, T),
}

pub enum ChannelAction<T: TPData> {
    Write(ChannelHandle<T>, T),
    Read(ChannelHandle<T>, T),
}

pub enum PropertyAction<T: TPData> {
    State(StateAction<T>),
    Channel(ChannelAction<T>),
}

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
