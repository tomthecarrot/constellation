use crate::contract::properties::{ChannelHandle, State, StateHandle, TPData};

pub enum PropertyAction<T: TPData> {
    StateWrite(StateHandle<T>, T),
    StateAssert(StateHandle<T>, T),
    ChannelWrite(ChannelHandle<T>, T),
}

pub trait Action<T: TPData>: Send + Sync {
    fn kind(&self) -> ActionKind;
    fn state_handle(&self) -> StateHandle<T>;
    fn raw_data(&self) -> &T;
    fn into_bytes(self) -> Box<[u8]>;
    // TODO[SER-257]: fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub type ActionResult<T> = Result<Box<dyn Action<T>>, Box<dyn Action<T>>>;

pub struct Collaction<T> {
    actions: Vec<Box<dyn Action<T>>>,
}

impl<T: TPData> Collaction<T> {
    pub fn actions(self) -> Vec<Box<dyn Action<T>>> {
        self.actions
    }
}

pub type CollactionResult<T> = Result<Collaction<T>, Collaction<T>>;

// ---- Action trait impls ----

impl<T: TPData> Action<T> for PropertyAction<T> {
    fn kind(&self) -> ActionKind {
        match self {
            PropertyAction::StateWrite(_, _) => ActionKind::StateWrite,
            PropertyAction::StateAssert(_, _) => ActionKind::StateAssert,
            PropertyAction::ChannelWrite(_, _) => ActionKind::ChannelWrite,
        }
    }

    fn state_handle(&self) -> StateHandle<T> {
        match self {
            PropertyAction::StateWrite(state_handle, _) => state_handle.clone(),
            PropertyAction::StateAssert(state_handle, _) => state_handle.clone(),
            _ => {
                panic!("[Action] Cannot return state handle: not a state!");
            }
        }
    }

    fn raw_data(&self) -> &T {
        match self {
            PropertyAction::StateWrite(_, raw_data) => raw_data,
            PropertyAction::StateAssert(_, raw_data) => raw_data,
            _ => {
                panic!("[Action] Cannot return raw data: not implemented!");
            }
        }
    }

    fn into_bytes(self) -> Box<[u8]> {
        todo!("Convert PropertyAction into bytes for transfer over the wire.");
    }
}

// ---- ObjectAction types ----

pub enum ActionKind {
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
