use crate::contract::properties::{ChannelHandle, State, StateHandle, TPData};

use eyre::Result;
use std::io::Write;

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

pub trait Action: Send + Sync {
    fn into_bytes(self) -> Box<[u8]>;
    // TODO[SER-257]: fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub struct Collaction {
    actions: Vec<Box<dyn Action>>,
}

pub struct CollactionResult {
    collaction: Collaction,
    was_accepted: bool,
}

// ---- Action trait impls ----

impl<T: TPData> Action for StateAction<T> {
    fn into_bytes(self) -> Box<[u8]> {
        todo!()
    }
}

impl<T: TPData> Action for ChannelAction<T> {
    fn into_bytes(self) -> Box<[u8]> {
        todo!()
    }
}

impl<T: TPData> Action for PropertyAction<T> {
    fn into_bytes(self) -> Box<[u8]> {
        todo!()
    }
}
