#[allow(unused)]
mod generated;
pub use self::generated::tp_serialize::*;

mod deserializer;
pub use self::deserializer::Deserializer;

mod serializer;
pub use self::serializer::Serializer;

mod types;

/// The types related to the tp_client
mod c {
    pub use tp_client::baseline::Baseline;
    pub use tp_client::contract::properties::dynamic::{TpPrimitiveType, TpPropertyType};
    pub use tp_client::contract::{Contract, ContractData, ContractDataHandle, ContractId};
    pub use tp_client::object::ObjectHandle;
}

/// The types related to the flatbuffer
mod t {
    pub use crate::baseline::Baseline;
    pub use crate::contract::{Contract, ContractDataHandle, ContractId, ContractStates};
    pub use crate::object::{Object, ObjectHandle};
    pub use crate::primitive::TpPrimitive;
    pub use crate::primitive::TpPrimitiveKind;
    pub use crate::state::{State, StateHandle};
    pub mod primitive {
        pub use crate::primitive::{Bool, String, F32, F64, I16, I32, I64, I8, U16, U32, U64, U8};
    }
}

pub const PREFIX: &str = "TPF1";
