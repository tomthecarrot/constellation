#![allow(non_camel_case_types, non_snake_case)]

#[rsharp::substitute("tp_contract_example")]
// Dummy module since inner macro attributes are unstable
mod _dummy {
    use rsharp::remangle;
    use safer_ffi::prelude::*;
    use tp_client::contract::properties::states::id::c_api::{
        StateId_ContractDataHandle, StateId_F32, StateId_I8, StateId_ObjectHandle, StateId_String,
        StateId_U8,
    };
    use tp_client::contract::{states, ContractDataHandle};
    use tp_client::object::ObjectHandle;

    #[remangle(substitute!())]
    #[states]
    #[derive_ReprC]
    #[ReprC::opaque]
    #[derive(Clone)]
    pub struct ExampleStates {
        u8_0: u8,
        u8_1: u8,
        i8_0: i8,
        i8_1: i8,
        f32_0: f32,
        f32_1: f32,
        str_0: String,
        oh_0: ObjectHandle,
        ch_0: ContractDataHandle,
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__u8_0(s: &ExampleStates) -> repr_c::Box<StateId_U8> {
        Box::new(StateId_U8::from(s.u8_0)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__u8_1(s: &ExampleStates) -> repr_c::Box<StateId_U8> {
        Box::new(StateId_U8::from(s.u8_1)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__i8_0(s: &ExampleStates) -> repr_c::Box<StateId_I8> {
        Box::new(StateId_I8::from(s.i8_0)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__i8_1(s: &ExampleStates) -> repr_c::Box<StateId_I8> {
        Box::new(StateId_I8::from(s.i8_1)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__f32_0(s: &ExampleStates) -> repr_c::Box<StateId_F32> {
        Box::new(StateId_F32::from(s.f32_0)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__f32_1(s: &ExampleStates) -> repr_c::Box<StateId_F32> {
        Box::new(StateId_F32::from(s.f32_1)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__str_0(s: &ExampleStates) -> repr_c::Box<StateId_String> {
        Box::new(StateId_String::from(s.str_0)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__oh_0(s: &ExampleStates) -> repr_c::Box<StateId_ObjectHandle> {
        Box::new(StateId_ObjectHandle::from(s.oh_0)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleStates__ch_0(s: &ExampleStates) -> repr_c::Box<StateId_ContractDataHandle> {
        Box::new(StateId_ContractDataHandle::from(s.ch_0)).into()
    }
}
pub use _dummy::*;
