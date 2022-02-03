mod primitive;
mod property;
mod vec;

// dyn_macro should not be used directly in the public API
#[doc(hidden)]
pub mod __macro;

pub use primitive::{DynTpPrimitive, TpPrimitiveType};
pub use property::{DynTpProperty, TpPropertyType};
pub use vec::DynTpVec;

pub use __macro::{apply_to_channel_id, apply_to_prop, apply_to_state_id};
