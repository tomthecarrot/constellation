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
    background_object: ObjectHandle,
    addressable_id: String,
}

#[cfg(feature = "c_api")]
mod c_api {
    #![allow(non_snake_case)]
    use super::*;

    mod _MKEBackgroundStates {
        use super::*;

        #[ffi_export]
        fn MKEBackgroundStates__background_object(
            s: &MKEBackgroundStates
        ) -> repr_c::Box<ObjectHandle> {
            repr_c::Box::new(s.background_object.into())
        }

        #[ffi_export]
        fn MKEBackgroundStates__addressable_id(s: &MKEBackgroundStates) -> repr_c:Box<String> {
            repr_c::Box::new(s.addressable_id.into())
        }
    }
}
