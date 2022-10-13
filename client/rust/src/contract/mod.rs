pub use tp_contract_macro::{channels, states};
pub mod properties;

use crate::contract::properties::channels::{ChannelsIter, IChannels};
use crate::contract::properties::states::{IStates, StatesIter};
use crate::object::ObjectHandle;

#[cfg(feature = "c_api")]
use safer_ffi::derive_ReprC;

use std::collections::HashSet;

/// Represents information that globally and uniquely identifies a contract.
/// Any two contracts are the same if their `ContractId`s are equal.
///
/// Due to the cost of comparing two `ContractId`s, [`ContractIdHandle`]s are
/// typically used instead.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "c_api",
    derive_ReprC,
    ReprC::opaque("tp_client__contract__ContractId")
)]
pub struct ContractId {
    pub name: &'static str,
    pub version: (u16, u16, u16),
}

pub type ContractDataHandle = arena::Index<ContractData>;

/// Contracts describe the valid set of properties in a category of objects, much
/// like a struct definition describes the variables in a particular instance of a
/// struct, or a class describes objects that are instances of that class.
///
/// Note that `Contract`s are not held internally by the [`Realm`], rather only
/// [`ContractData`]s and [`ContractDataHandle`]s. The API client should therefore hold
/// onto the instantiated `Contract` so that they can access the fields of any
/// particular object.
pub trait Contract {
    type States: IStates;
    type Channels: IChannels;

    const ID: ContractId;

    fn new(handle: ContractDataHandle) -> Self;

    fn states(&self) -> &Self::States;
    fn channels(&self) -> &Self::Channels;
    fn handle(&self) -> ContractDataHandle;
    fn state_iter(&self) -> StatesIter<Self::States> {
        StatesIter::new(self.handle())
    }
    fn chan_iter(&self) -> ChannelsIter<Self::Channels> {
        ChannelsIter::new(self.handle())
    }
}

/// Contains stateful data about the contract
#[cfg_attr(
    feature = "c_api",
    derive_ReprC,
    ReprC::opaque("tp_client__contract__ContractData")
)]
pub struct ContractData {
    id: ContractId,
    objects: HashSet<ObjectHandle>,
}
impl ContractData {
    pub fn new(id: ContractId) -> Self {
        Self {
            id,
            objects: Default::default(),
        }
    }
    pub fn id(&self) -> ContractId {
        self.id
    }

    pub(super) fn objects_mut(&mut self) -> &mut HashSet<ObjectHandle> {
        &mut self.objects
    }

    pub fn objects(&self) -> &HashSet<ObjectHandle> {
        &self.objects
    }
}

#[cfg(feature = "c_api")]
#[rsharp::substitute("tp_client::contract")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]

    use super::{ContractData, ContractId};
    use crate::contract::properties::c_api::c_types;

    use rsharp::remangle;
    use safer_ffi::prelude::*;

    mod _ContractDataHandle {
        use crate::contract::properties::c_api::impl_from_refcast;
        use crate::contract::ContractDataHandle as RContractDataHandle;

        use derive_more::{From, Into};
        use ref_cast::RefCast;
        use rsharp::remangle;
        use safer_ffi::prelude::*;

        #[remangle(substitute!())]
        #[derive_ReprC]
        #[ReprC::opaque]
        #[derive(Clone, Copy, Eq, PartialEq, From, Into, RefCast)]
        #[repr(C)]
        pub struct ContractDataHandle {
            pub inner: RContractDataHandle,
        }
        impl_from_refcast!(RContractDataHandle, ContractDataHandle);

        #[remangle(substitute!())]
        #[ffi_export]
        pub fn ContractDataHandle__drop(c: repr_c::Box<ContractDataHandle>) {
            drop(c)
        }
    }
    pub use _ContractDataHandle::ContractDataHandle;

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ContractData__id<'a>(cd: &'a ContractData) -> &'a ContractId {
        &cd.id
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ContractData__objects<'a>(cd: &'a ContractData) -> repr_c::Vec<c_types::ObjectHandle> {
        // TODO: Avoid allocation/copy
        let v: Vec<_> = cd
            .objects
            .iter()
            .map(|h| c_types::ObjectHandle::from(*h))
            .collect();
        v.into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ContractId__name<'a>(cid: &'a ContractId) -> str::Ref<'static> {
        cid.name.into()
    }

    #[remangle(substitute!())]
    #[derive_ReprC]
    #[repr(C)]
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct ContractId_Version {
        pub major: u16,
        pub minor: u16,
        pub patch: u16,
    }
    impl From<(u16, u16, u16)> for ContractId_Version {
        fn from(o: (u16, u16, u16)) -> Self {
            Self {
                major: o.0,
                minor: o.1,
                patch: o.2,
            }
        }
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ContractId__version<'a>(cid: &'a ContractId) -> ContractId_Version {
        cid.version.into()
    }

    mod _ExampleStates {
        use crate::contract::properties::states::id::c_api::{
            StateId_F32, StateId_I8, StateId_String, StateId_U8,
        };
        use crate::contract::states;

        use rsharp::remangle;
        use safer_ffi::prelude::*;

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
    }
    pub use _ExampleStates::ExampleStates;

    mod _ExampleContract {
        use super::ContractDataHandle as CContractDataHandle;
        use super::ExampleStates;
        use crate::baseline::Baseline;
        use crate::contract::properties::dynamic::DynTpProperty;
        use crate::contract::{Contract, ContractDataHandle, ContractId};
        use crate::object::c_api::ObjectHandle as CObjectHandle;

        use rsharp::remangle;
        use safer_ffi::prelude::*;

        #[remangle(substitute!())]
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
    }
    pub use _ExampleContract::ExampleContract;
}
