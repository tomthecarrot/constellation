#![allow(non_camel_case_types, non_snake_case)]

mod states;
pub use crate::states::ExampleStates;

use rsharp::remangle;
use safer_ffi::prelude::*;
use tp_client::contract::{Contract, ContractDataHandle, ContractId};

#[remangle("tp_contract_example")]
#[derive_ReprC]
#[ReprC::opaque]
pub struct ExampleContract {
    handle: ContractDataHandle,
    states: ExampleStates,
    channels: (),
}
impl Contract for ExampleContract {
    type States = ExampleStates;

    type Channels = ();

    const ID: ContractId = ContractId {
        name: "teleportal.example-ffi-contract",
        version: (1, 2, 3),
    };

    fn new(handle: ContractDataHandle) -> Self {
        Self {
            handle,
            states: ExampleStates::new(handle),
            channels: (),
        }
    }

    fn states(&self) -> &Self::States {
        &self.states
    }

    fn channels(&self) -> &Self::Channels {
        &self.channels
    }

    fn handle(&self) -> ContractDataHandle {
        self.handle
    }
}

#[rsharp::substitute("tp_contract_example")]
mod c_api {
    use super::*;
    use tp_client::baseline::Baseline;
    use tp_client::contract::c_api::ContractDataHandle as CContractDataHandle;
    use tp_client::contract::properties::dynamic::DynTpProperty;
    use tp_client::contract::Contract;
    use tp_client::object::c_api::ObjectHandle as CObjectHandle;

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleContract__drop(c: repr_c::Box<ExampleContract>) {
        drop(c)
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleContract__handle(c: &ExampleContract) -> repr_c::Box<CContractDataHandle> {
        Box::new(CContractDataHandle::from(c.handle())).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleContract__states<'a>(c: &'a ExampleContract) -> &'a ExampleStates {
        c.states()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleContract__object_create(
        contract: &ExampleContract,
        baseline: &mut Baseline,
        u8_0: u8,
        u8_1: u8,
        i8_0: i8,
        i8_1: i8,
        f32_0: f32,
        f32_1: f32,
    ) -> repr_c::Box<CObjectHandle> {
        let states = [
            DynTpProperty::Primitive(u8_0.into()),
            DynTpProperty::Primitive(u8_1.into()),
            DynTpProperty::Primitive(i8_0.into()),
            DynTpProperty::Primitive(i8_1.into()),
            DynTpProperty::Primitive(f32_0.into()),
            DynTpProperty::Primitive(f32_1.into()),
        ];
        let obj = baseline
            .object_create(contract, states.into_iter(), [].into_iter())
            .unwrap();
        Box::new(CObjectHandle::from(obj)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleContract__object_remove(
        baseline: &mut Baseline,
        obj: repr_c::Box<CObjectHandle>,
    ) {
        baseline
            .object_remove::<ExampleContract>(obj.inner)
            .unwrap()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ExampleContract__register<'a>(
        baseline: &'a mut Baseline,
    ) -> repr_c::Box<ExampleContract> {
        let c: ExampleContract = baseline
            .register_contract()
            .expect("Failed to register contract");
        Box::new(c).into()
    }
}

// This generates the C header file for the bindings. See safer-ffi's guide.
#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    let builder = ::safer_ffi::headers::builder();
    if ::std::env::var("HEADERS_TO_STDOUT")
        .ok()
        .map_or(false, |it| it == "1")
    {
        builder.to_writer(::std::io::stdout()).generate()
    } else {
        builder.to_file(&"generated.h".to_string())?.generate()
    }
}
