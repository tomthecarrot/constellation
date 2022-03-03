use crate::contract::properties::channels::ChannelHandle;
use crate::contract::properties::states::StateHandle;
use crate::contract::properties::traits::ITpProperty;

use crate::action::{ActionKind, IAction};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(IAction)]
pub enum PropertyAction<T: ITpProperty> {
    State(StateAction<T>),
    Channel(ChannelAction<T>),
}

pub enum ChannelAction<T: ITpProperty> {
    Assert { handle: ChannelHandle<T>, data: T },
    Write { handle: ChannelHandle<T>, data: T },
}
pub enum StateAction<T: ITpProperty> {
    Assert { handle: StateHandle<T>, data: T },
    Write { handle: StateHandle<T>, data: T },
}

impl<T: ITpProperty> IAction for StateAction<T> {
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

impl<T: ITpProperty> IAction for ChannelAction<T> {
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
