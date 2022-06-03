pub use tp_contract_macro::{channels, states};
pub mod properties;

use crate::contract::properties::channels::{ChannelsIter, IChannels};
use crate::contract::properties::states::{IStates, StatesIter};
use crate::object::ObjectHandle;

#[cfg(feature = "safer-ffi")]
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
    feature = "safer-ffi",
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

#[cfg_attr(feature = "c_api", rsharp::substitute("tp_client::contract"))]
#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]
    use super::{ContractData, ContractId};
    use crate::contract::properties::c_api::c_types;
    use crate::contract::properties::c_api::impl_from_refcast;

    use derive_more::{From, Into};
    use ref_cast::RefCast;
    use safer_ffi::prelude::*;

    use rsharp::remangle;

    #[remangle(substitute!())]
    #[derive_ReprC]
    #[ReprC::opaque]
    #[derive(Clone, Copy, Eq, PartialEq, From, Into, RefCast)]
    #[repr(C)]
    pub struct ContractDataHandle {
        pub inner: super::ContractDataHandle,
    }
    impl_from_refcast!(super::ContractDataHandle, ContractDataHandle);

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn ContractDataHandle__drop(c: repr_c::Box<ContractDataHandle>) {
        drop(c)
    }

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

    // TODO: expose ContractId version info
}
