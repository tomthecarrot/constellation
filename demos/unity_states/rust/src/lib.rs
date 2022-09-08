#![deny(
    bad_style,
    const_err,
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
use safer_ffi::derive_ReprC;

use tp_client::contract::{states, Contract, ContractDataHandle, ContractId};

#[derive_ReprC]
#[ReprC::opaque]
pub struct BallContract {
    handle: ContractDataHandle,
    states: BallStates,
}
impl Contract for BallContract {
    type States = BallStates;

    type Channels = ();

    const ID: ContractId = ContractId {
        name: "Ball",
        version: (0, 0, 0),
    };

    fn new(handle: ContractDataHandle) -> Self {
        Self {
            handle,
            states: BallStates::new(handle),
        }
    }

    fn states(&self) -> &Self::States {
        &self.states
    }

    fn channels(&self) -> &Self::Channels {
        &()
    }

    fn handle(&self) -> tp_client::contract::ContractDataHandle {
        self.handle
    }
}

#[states]
#[derive_ReprC]
#[ReprC::opaque]
pub struct BallStates {
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    euler_x: i16,
    euler_y: i16,
    euler_z: i16,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    color: u64,
}

mod c_api {
    #![allow(non_snake_case)]
    use super::*;

    use safer_ffi::prelude::*;
    use tp_client::baseline::Baseline;
    use tp_client::contract::c_api::ContractDataHandle as CContractDataHandle;
    use tp_client::contract::properties::states::c_api::{StateId_F32, StateId_I16, StateId_U64};
    use tp_client::object::c_api::ObjectHandle as CObjectHandle;

    mod _BallStates {
        use super::*;

        #[ffi_export]
        fn BallStates__pos_x(s: &BallStates) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.pos_x.into())
        }

        #[ffi_export]
        fn BallStates__pos_y(s: &BallStates) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.pos_y.into())
        }

        #[ffi_export]
        fn BallStates__pos_z(s: &BallStates) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.pos_z.into())
        }

        #[ffi_export]
        fn BallStates__euler_x(s: &BallStates) -> repr_c::Box<StateId_I16> {
            repr_c::Box::new(s.euler_x.into())
        }

        #[ffi_export]
        fn BallStates__euler_y(s: &BallStates) -> repr_c::Box<StateId_I16> {
            repr_c::Box::new(s.euler_y.into())
        }

        #[ffi_export]
        fn BallStates__euler_z(s: &BallStates) -> repr_c::Box<StateId_I16> {
            repr_c::Box::new(s.euler_z.into())
        }

        #[ffi_export]
        fn BallStates__scale_x(s: &BallStates) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.scale_x.into())
        }

        #[ffi_export]
        fn BallStates__scale_y(s: &BallStates) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.scale_y.into())
        }

        #[ffi_export]
        fn BallStates__scale_z(s: &BallStates) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.scale_z.into())
        }

        #[ffi_export]
        fn BallStates__color(s: &BallStates) -> repr_c::Box<StateId_U64> {
            repr_c::Box::new(s.color.into())
        }
    }

    mod _BallContract {
        use super::*;
        #[ffi_export]
        fn BallContract__register(baseline: &mut Baseline) -> repr_c::Box<BallContract> {
            repr_c::Box::new(baseline.register_contract().unwrap())
        }

        #[ffi_export]
        fn BallContract__unregister(
            baseline: &mut Baseline,
            contract: repr_c::Box<CContractDataHandle>,
        ) {
            baseline
                .unregister_contract::<BallContract>(contract.inner)
                .unwrap()
        }

        #[ffi_export]
        fn BallContract__drop(contract: repr_c::Box<BallContract>) {
            drop(contract)
        }

        #[ffi_export]
        fn BallContract__handle(c: &BallContract) -> repr_c::Box<CContractDataHandle> {
            repr_c::Box::new(c.handle().into())
        }

        #[ffi_export]
        fn BallContract__states<'a>(c: &'a BallContract) -> &'a BallStates {
            c.states()
        }

        #[ffi_export]
        fn BallContract__object_create(
            baseline: &mut Baseline,
            contract: &BallContract,
            pos_x: f32,
            pos_y: f32,
            pos_z: f32,
            euler_x: i16,
            euler_y: i16,
            euler_z: i16,
            scale_x: f32,
            scale_y: f32,
            scale_z: f32,
            color: u64,
        ) -> repr_c::Box<CObjectHandle> {
            use tp_client::contract::properties::dynamic::DynTpProperty;

            let states = [
                DynTpProperty::Primitive(pos_x.into()),
                DynTpProperty::Primitive(pos_y.into()),
                DynTpProperty::Primitive(pos_z.into()),
                DynTpProperty::Primitive(euler_x.into()),
                DynTpProperty::Primitive(euler_y.into()),
                DynTpProperty::Primitive(euler_z.into()),
                DynTpProperty::Primitive(scale_x.into()),
                DynTpProperty::Primitive(scale_y.into()),
                DynTpProperty::Primitive(scale_z.into()),
                DynTpProperty::Primitive(color.into()),
            ];
            repr_c::Box::new(
                baseline
                    .object_create(contract, states.into_iter(), [].into_iter())
                    .unwrap()
                    .into(),
            )
        }

        #[ffi_export]
        fn BallContract__object_remove(baseline: &mut Baseline, obj: repr_c::Box<CObjectHandle>) {
            baseline.object_remove::<BallContract>(obj.inner).unwrap()
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
}
