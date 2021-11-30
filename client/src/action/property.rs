use crate::contract::properties::{ChannelHandle, StateHandle, TPData};

use crate::action::{ActionKind, TAction};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(TAction)]
pub enum PropertyAction<T: TPData> {
    State(StateAction<T>),
    Channel(ChannelAction<T>),
}

pub enum ChannelAction<T: TPData> {
    Assert { handle: ChannelHandle<T>, data: T },
    Write { handle: ChannelHandle<T>, data: T },
}
pub enum StateAction<T: TPData> {
    Assert { handle: StateHandle<T>, data: T },
    Write { handle: StateHandle<T>, data: T },
}

impl<T: TPData> TAction for StateAction<T> {
    fn kind(&self) -> ActionKind {
        match self {
            Self::Write { .. } => ActionKind::StateWrite,
            Self::Assert { .. } => ActionKind::StateAssert,
        }
    }

    fn into_bytes(self) -> Box<[u8]> {
        todo!();
    }
}

impl<T: TPData> TAction for ChannelAction<T> {
    fn kind(&self) -> ActionKind {
        match self {
            Self::Write { .. } => ActionKind::ChannelWrite,
            Self::Assert { .. } => ActionKind::ChannelAssert,
        }
    }

    fn into_bytes(self) -> Box<[u8]> {
        todo!();
    }
}
