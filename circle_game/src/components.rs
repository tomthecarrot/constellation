use bevy::prelude::Component;
use derive_more::From;

// Local reexport for convenience
pub mod tp {
    pub use tp_client::baseline::BaselineKind;
    pub use tp_client::object::ObjectHandle;
}

#[derive(Component, From)]
pub struct BaselineKind(pub tp::BaselineKind);

#[derive(Component, From)]
pub struct ObjectHandle(pub tp::ObjectHandle);
