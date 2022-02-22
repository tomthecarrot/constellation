mod borrow_impls;
mod eq_impls;
mod from_impls;
mod primitive;
mod property;
mod vec;

// dyn_macro should not be used directly in the public API
#[doc(hidden)]
pub mod __macro;

pub use primitive::{DynTpPrimitive, TpPrimitiveType};
pub use property::{DynTpProperty, DynTpPropertyMut, DynTpPropertyRef};
pub use vec::DynTpVec;

pub use __macro::{apply_to_channel_id, apply_to_prop, apply_to_state_id};

/// The static type of the ITpPropertyStatic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpPropertyType {
    Vec(TpPrimitiveType),
    Primitive(TpPrimitiveType),
}
impl TpPropertyType {
    pub const fn primitive_type(&self) -> TpPrimitiveType {
        match self {
            Self::Vec(pt) => *pt,
            Self::Primitive(pt) => *pt,
        }
    }
}
