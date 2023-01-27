#![deny(
    bad_style,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

dynpath::wrap! {
    #[allow(unused)]
    #[dynpath("OUT_DIR", "mod.rs")]
    mod generated;
}
pub use self::generated::tp_serialize::*;

mod deserializer;
pub use self::deserializer::Deserializer;

mod serializer;
pub use self::serializer::Serializer;

mod types;

/// The types related to the tp_client rust library
mod rs {
    pub use tp_client::baseline::{Baseline, BaselineKind};
    pub use tp_client::contract::properties::dynamic::{TpPrimitiveType, TpPropertyType};
    pub use tp_client::contract::properties::states::{
        DynStateHandle, State, StateHandle, StateId,
    };
    pub use tp_client::contract::{Contract, ContractData, ContractDataHandle, ContractId};
    pub use tp_client::object::ObjectHandle;
}

/// The types related to the flatbuffer
mod fb {
    pub use crate::baseline::Baseline;
    pub use crate::contract::{Contract, ContractDataHandle, ContractId, ContractStates};
    pub use crate::object::{Object, ObjectHandle};
    pub use crate::primitive::TpPrimitive;
    pub use crate::primitive::TpPrimitiveKind;
    pub use crate::state::{State, StateHandle};
    pub mod primitive {
        pub use crate::primitive::{
            Bool, FbString, F32, F64, I16, I32, I64, I8, U16, U32, U64, U8,
        };
    }
}

pub const PREFIX: &str = "TPF1";
