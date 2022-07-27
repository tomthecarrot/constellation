#[cfg(feature = "c_api")]
use safer_ffi::derive_ReprC;

use tp_client::{
    contract::{states, Contract, ContractDataHandle, ContractId},
    object::ObjectHandle,
};

#[cfg_attr(feature = "safer-ffi", derive_ReprC, ReprC::opaque)]
pub struct MKEBackgroundContract {
    handle: ContractDataHandle,
    states: MKEBackgroundStates,
}
impl Contract for MKEBackgroundContract {
    type States = MKEBackgroundStates;

    type Channels = ();

    const ID: ContractId = ContractId {
        name: "Background",
        version: (0, 0, 0),
    };

    fn new(handle: ContractDataHandle) -> Self {
        Self {
            handle: handle,
            states: MKEBackgroundStates::new(handle),
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
#[cfg_attr(feature = "safer-ffi", derive_ReprC, ReprC::opaque)]
pub struct MKEBackgroundStates {
    background_object: f32,
}

#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_snake_case)]
    use super::*;
    use tp_client::baseline::Baseline;
    use tp_client::contract::c_api::ContractDataHandle as CContractDataHandle;
    use tp_client::contract::properties::states::c_api::StateId_F32;
    use tp_client::object::c_api::ObjectHandle as CObjectHandle;

    use safer_ffi::prelude::*;

    pub mod _MKEBackgroundStates {
        use super::*;

        #[ffi_export]
        fn MKEBackgroundStates__background_object(
            s: &MKEBackgroundStates,
        ) -> repr_c::Box<StateId_F32> {
            repr_c::Box::new(s.background_object.into())
        }
    }

    pub mod _MKEBackgroundContract {
        use tp_client::contract::properties::dynamic::DynTpProperty;

        use super::*;

        #[ffi_export]
        fn MKEBackgroundContract__register(
            baseline: &mut Baseline,
        ) -> repr_c::Box<MKEBackgroundContract> {
            repr_c::Box::new(baseline.register_contract().unwrap())
        }

        #[ffi_export]
        fn MKEBackgroundContract__drop(contract: repr_c::Box<MKEBackgroundContract>) {
            drop(contract)
        }

        #[ffi_export]
        fn MKEBackgroundContract__handle(
            c: &MKEBackgroundContract,
        ) -> repr_c::Box<CContractDataHandle> {
            repr_c::Box::new(c.handle().into())
        }

        #[ffi_export]
        fn MKEBackgroundContract__states<'a>(
            c: &'a MKEBackgroundContract,
        ) -> &'a MKEBackgroundStates {
            c.states()
        }

        #[ffi_export]
        fn MKEBackgroundContract__object_create(
            baseline: &mut Baseline,
            contract: &MKEBackgroundContract,
            background_object: f32,
            // addressable_id: String,
        ) -> repr_c::Box<CObjectHandle> {
            let states = [
                DynTpProperty::Primitive(background_object.into()),
                // DynTpProperty::Primitive(addressable_id.into()),
            ];
            repr_c::Box::new(
                baseline
                    .object_create(contract, states.into_iter(), [].into_iter())
                    .unwrap()
                    .into(),
            )
        }
    }
}
