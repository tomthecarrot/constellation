use crate::contract::properties::{ChannelHandle, State, StateHandle, TPData};

pub enum PropertyAction<T: TPData> {
    StateWrite(StateHandle<T>, T),
    StateAssert(StateHandle<T>, T),
    ChannelWrite(ChannelHandle<T>, T),
}

pub trait Action: Send + Sync {
    fn get_type(&self) -> ActionType;
    fn into_bytes(self) -> Box<[u8]>;
    // TODO[SER-257]: fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub type ActionResult = bool;

pub struct Collaction {
    actions: Vec<Box<dyn Action>>,
}

pub struct CollactionResult {
    collaction: Collaction,
    is_approved: bool,
}

impl Collaction {
    pub fn get_actions(&self) -> &Vec<Box<dyn Action>> {
        &self.actions
    }
}

impl CollactionResult {
    pub fn new(collaction: Collaction, is_approved: bool) -> Self {
        Self {
            collaction,
            is_approved,
        }
    }
}

// ---- Action trait impls ----

impl<T: TPData> Action for PropertyAction<T> {
    fn get_type(&self) -> ActionType {
        match self {
            PropertyAction::StateWrite(_, _) => ActionType::StateWrite,
            PropertyAction::StateAssert(_, _) => ActionType::StateAssert,
            PropertyAction::ChannelWrite(_, _) => ActionType::ChannelWrite,
        }
    }

    fn into_bytes(self) -> Box<[u8]> {
        todo!("Convert PropertyAction into bytes for transfer over the wire.");
    }
}

// ---- ObjectAction types ----

pub enum ActionType {
    StateWrite,
    StateIncrement,
    QueueWrite,
    QueueIncrement,
    ChannelWrite,
    ChannelCommit,
    ObjectArm,
    ObjectRtPreviewEnable,
    TimeWrite,
    Lock,
    StateAssert,
    QueueAssert,
}
