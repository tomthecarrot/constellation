use crate::contract::properties::channels::{ChannelHandle, ChannelId};
use crate::contract::properties::states::{StateHandle, StateId};
use crate::contract::properties::traits::{ITpData, ITpPropertyStatic};
use crate::contract::ContractDataHandle;
use crate::time::TimeWarp;

use arena::generational_arena as ga;
use eyre::{eyre, Result};

// TODO: Can we handle mapping from StateID -> StateHandle more sanely?

#[cfg_attr(feature = "c_api", safer_ffi::derive_ReprC, ReprC::opaque)]
pub struct Object {
    // we have to store type erased index here to get around unsized types
    states: Vec<ga::Index>,   // map from StateID -> StateHandle
    channels: Vec<ga::Index>, // map from ChannelID -> ChannelHandle
    contract: ContractDataHandle,
    time_warp: TimeWarp,
}
impl Object {
    pub(crate) fn new(
        states: Vec<ga::Index>,
        channels: Vec<ga::Index>,
        contract: ContractDataHandle,
        time_warp: TimeWarp,
    ) -> Self {
        Self {
            states,
            channels,
            contract,
            time_warp,
        }
    }

    pub fn state<T: ITpData>(&self, id: StateId<T>) -> StateHandle<T> {
        assert_eq!(
            id.contract(),
            self.contract,
            "`id` was for a different contract!"
        );
        let idx = self.states.get(id.idx());
        let idx = if let Some(idx) = idx {
            *idx
        } else {
            unreachable!(
                "Because the `StateID` comes from the contract, and we have already
                verified that the contract matches, it should not be possible for
                the index to not be valid"
            )
        };
        StateHandle::new(idx)
    }

    pub fn channel<T: ITpData>(&self, id: ChannelId<T>) -> ChannelHandle<T> {
        assert_eq!(
            id.contract(),
            self.contract,
            "`id` was for a different contract!"
        );
        let idx = self.channels.get(id.idx());
        let idx = if let Some(idx) = idx {
            *idx
        } else {
            unreachable!(
                "Because the `ChannelID` comes from the contract, and we have already
                verified that the contract matches, it should not be possible for
                the index to not be valid"
            )
        };
        ChannelHandle::new(idx)
    }

    pub fn contract(&self) -> ContractDataHandle {
        self.contract
    }

    pub fn time_warp(&self) -> &TimeWarp {
        &self.time_warp
    }

    pub fn time_warp_mut(&mut self) -> &mut TimeWarp {
        &mut self.time_warp
    }

    pub(crate) fn bind_state<T: ITpPropertyStatic>(
        &self,
        id: StateId<T>,
    ) -> Result<StateHandle<T>> {
        if id.contract() != self.contract() {
            return Err(eyre!("Supplied id did not match this object's contract"));
        }
        let idx = *self.states.get(id.idx()).ok_or_else(|| {
            eyre!("id's field index out of bounds for this object. Should have been impossible!")
        })?;

        let handle: StateHandle<T> = arena::Index::new(idx);
        Ok(handle)
    }

    pub(crate) fn bind_channel<T: ITpPropertyStatic>(
        &self,
        id: ChannelId<T>,
    ) -> Result<ChannelHandle<T>> {
        if id.contract() != self.contract() {
            return Err(eyre!("Supplied id did not match this object's contract"));
        }
        let idx = *self.channels.get(id.idx()).ok_or_else(|| {
            eyre!("id's field index out of bounds for this object. Should have been impossible!")
        })?;

        let handle: ChannelHandle<T> = arena::Index::new(idx);
        Ok(handle)
    }
}

pub type ObjectHandle = arena::Index<Object>;

#[cfg(feature = "c_api")]
#[rsharp::substitute("tp_client::object")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]

    use crate::contract::properties::c_api::impl_from_refcast;

    use derive_more::{From, Into};
    use ref_cast::RefCast;
    use rsharp::remangle;
    use safer_ffi::prelude::*;

    #[remangle(substitute!())]
    #[derive(From, Into, Copy, Clone, RefCast)]
    #[derive_ReprC]
    #[ReprC::opaque]
    #[repr(C)]
    pub struct ObjectHandle {
        pub inner: super::ObjectHandle,
    }
    impl_from_refcast!(super::ObjectHandle, ObjectHandle);
}
