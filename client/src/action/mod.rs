pub mod property;

use crate::contract::properties::{ChannelHandle, StateHandle, TPData};
use property::{ChannelAction, PropertyAction, StateAction};

use enum_dispatch::enum_dispatch;

#[enum_dispatch(TAction)]
pub enum Action<T: TPData = Box<dyn TPData>> {
    Property(PropertyAction<T>),
}

/// All `Action` variants satisfy `TAction` trait
#[enum_dispatch]
pub trait TAction {
    fn kind(&self) -> ActionKind;
    fn into_bytes(self) -> Box<[u8]>;
    // TODO[SER-257]: fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub type ActionResult = Result<(), ()>;

pub struct Collaction {
    actions: Vec<Action>,
}

impl Collaction {
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    pub fn actions_mut(&mut self) -> &mut [Action] {
        &mut self.actions
    }
}

pub type CollactionResult = Result<Collaction, Collaction>;

// ---- ObjectAction types ----
pub enum ActionKind {
    StateWrite,
    StateIncrement,
    QueueWrite,
    QueueIncrement,
    ChannelWrite,
    ChannelAssert,
    ChannelCommit,
    ObjectArm,
    ObjectRtPreviewEnable,
    TimeWrite,
    Lock,
    StateAssert,
    QueueAssert,
}
