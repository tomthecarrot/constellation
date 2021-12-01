use crate::contract::properties::{ChannelHandle, ChannelID, ITpData, StateHandle, StateID};
use crate::contract::ContractHandle;

use arena::generational_arena as ga;

// TODO: Can we handle mapping from StateID -> StateHandle more sanely?

pub struct Object {
    // we have to store type erased index here to get around unsized types
    states: Vec<ga::Index>,   // map from StateID -> StateHandle
    channels: Vec<ga::Index>, // map from ChannelID -> ChannelHandle
    contract: ContractHandle,
}
impl Object {
    pub fn state<T: ITpData>(&self, id: StateID) -> StateHandle<T> {
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

    pub fn channel<T: ITpData>(&self, id: ChannelID) -> ChannelHandle<T> {
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

    pub fn contract(&self) -> ContractHandle {
        self.contract
    }
}

pub type ObjectHandle = arena::Index<Object>;
