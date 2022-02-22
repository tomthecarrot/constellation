use crate::contract::properties::channels::{ChannelHandle, ChannelId};
use crate::contract::properties::states::{StateHandle, StateId};
use crate::contract::properties::traits::{ITpData, ITpPropertyStatic};
use crate::contract::ContractDataHandle;

use arena::generational_arena as ga;
use eyre::{eyre, Result};

// TODO: Can we handle mapping from StateID -> StateHandle more sanely?

pub struct Object {
    // we have to store type erased index here to get around unsized types
    states: Vec<ga::Index>,   // map from StateID -> StateHandle
    channels: Vec<ga::Index>, // map from ChannelID -> ChannelHandle
    contract: ContractDataHandle,
}
impl Object {
    pub(crate) fn new(
        states: Vec<ga::Index>,
        channels: Vec<ga::Index>,
        contract: ContractDataHandle,
    ) -> Self {
        Self {
            states,
            channels,
            contract,
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
