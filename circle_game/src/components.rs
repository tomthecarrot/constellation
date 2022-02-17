use bevy::prelude::Component;
use derive_more::From;

// Local reexport for convenience
pub mod tp {
    pub use tp_client::baseline::BaselineKind;
    pub use tp_client::contract::properties::state::StateHandle;
    pub use tp_client::object::ObjectHandle;
}

#[derive(Component, From, Clone, Copy, Debug)]
pub struct BaselineKind(pub tp::BaselineKind);

#[derive(Component, From, Clone, Copy, Debug)]
pub struct ObjectHandle(pub tp::ObjectHandle);

#[derive(Component, From, Clone, Copy, Debug)]
pub struct PosHandle {
    pub x: tp::StateHandle<f32>,
    pub y: tp::StateHandle<f32>,
}
