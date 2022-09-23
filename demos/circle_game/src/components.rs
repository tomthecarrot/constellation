use bevy::prelude::Component;
use derive_more::From;

// Local reexport for convenience
pub mod tp {
    pub use constellation::baseline::BaselineKind;
    pub use constellation::contract::properties::states::StateHandle;
    pub use constellation::object::ObjectHandle;
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
