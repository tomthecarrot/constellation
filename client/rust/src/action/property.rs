use crate::contract::properties::channels::DynChannelHandle;
use crate::contract::properties::dynamic::DynTpProperty;
use crate::contract::properties::states::DynStateHandle;

use crate::action::{ActionKind, IAction};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(IAction)]
pub enum PropertyAction {
    State(StateAction),
    Channel(ChannelAction),
}

pub enum ChannelAction {
    Assert {
        handle: DynChannelHandle,
        data: DynTpProperty,
    },
    Write {
        handle: DynChannelHandle,
        data: DynTpProperty,
    },
}
pub enum StateAction {
    Assert {
        handle: DynStateHandle,
        data: DynTpProperty,
    },
    Write {
        handle: DynStateHandle,
        data: DynTpProperty,
    },
}

impl IAction for StateAction {
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

impl IAction for ChannelAction {
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
