// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::contract::properties::channel::{
    apply_to_channel, Channel, ChannelArenaHandle, ChannelArenaMap, ChannelHandle, ChannelId,
    ChannelsIter, DynChannelId,
};
use crate::contract::properties::state::{
    apply_to_state, DynStateId, State, StateArenaHandle, StateArenaMap, StateHandle, StateId,
    StatesIter,
};

use crate::contract::properties::traits::{ITpProperty, ITpPropertyStatic};
use crate::contract::{Contract, ContractData, ContractDataHandle};
use crate::object::{Object, ObjectHandle};

use arena::Arena;
use eyre::{eyre, Result};
use typemap::ShareMap;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum BaselineKind {
    Main,
    Fork,
}
pub struct Baseline {
    kind: BaselineKind,
    objects: Arena<Object>,
    contracts: Arena<ContractData>,
    states: StateArenaMap,     // maps from T to Arena<State<T>>
    channels: ChannelArenaMap, // maps from T to Arena<Channel<T>>
}

impl Baseline {
    pub fn new(kind: BaselineKind) -> Self {
        let objects = Arena::new();
        let contracts = Arena::new();
        let states = ShareMap::custom();
        let channels = ShareMap::custom();

        Self {
            kind,
            objects,
            contracts,
            states,
            channels,
        }
    }

    // ---- Called by the Baseline on its fork ----

    // TODO[SER-259]: determine method for notifying Baseline fork.

    fn on_state_change<T: ITpProperty>(&self, state: StateHandle<T>) {
        todo!("Notify fork");
    }

    fn on_channel_change<T: ITpProperty>(&self, channel: ChannelHandle<T>) {
        todo!("Notify fork");
    }

    // ---- Object and Contract Acessors ----

    pub fn register_contract<C: Contract>(&mut self) -> eyre::Result<C> {
        for (_, c_data) in self.contracts.iter() {
            let c_id = c_data.id();
            if c_id == C::ID {
                return Err(eyre!("Contract already added!"));
            }
        }
        let handle = self.contracts.insert(ContractData::new(C::ID));
        Ok(C::new(handle))
    }

    pub fn unregister_contract<C: Contract>(
        &mut self,
        handle: ContractDataHandle,
    ) -> eyre::Result<()> {
        let c_data = self
            .contracts
            .get_mut(handle)
            .ok_or_else(|| eyre!("There is no contract with that id to unregister!"))?;

        if c_data.id() != C::ID {
            return Err(eyre!("Handle did not match the provided contract type!"));
        }

        // Its ok to steal the hashmap because c_data will be deleted soon anyway
        let objs = std::mem::take(c_data.objects_mut());
        for o in objs {
            self.object_remove::<C>(o)
                .expect("Failed to remove object!")
        }
        self.contracts.remove(handle);
        Ok(())
    }

    pub fn contract_data(&self, handle: ContractDataHandle) -> eyre::Result<&ContractData> {
        self.contracts
            .get(handle)
            .ok_or_else(|| eyre!("No contract exists for that handle!"))
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = (ObjectHandle, &Object)> {
        self.objects.iter()
    }

    pub fn object(&self, obj: ObjectHandle) -> eyre::Result<&Object> {
        self.objects
            .get(obj)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn object_mut(&mut self, obj: ObjectHandle) -> eyre::Result<&mut Object> {
        self.objects
            .get_mut(obj)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn object_add<C: Contract>(&mut self) -> eyre::Result<ObjectHandle> {
        todo!("Implement object addition")
    }

    pub fn object_remove<C: Contract>(&mut self, obj: ObjectHandle) -> eyre::Result<()> {
        let o = if let Some(o) = self.objects.remove(obj) {
            o
        } else {
            return Err(eyre!("Object did not exist, so it could not be removed"));
        };

        // remove all fields of the object
        let states = StatesIter::<C::States>::new(o.contract());
        let channels = ChannelsIter::<C::Channels>::new(o.contract());

        for s in states {
            apply_to_state!(s, |id| {
                let handle = self.bind_state(id, obj)?;
                if let Err(e) = self.state_remove(handle) {
                    log::warn!("Failed to remove state, state has been leaked: {}", e);
                }
                Ok::<(), eyre::Report>(())
            })?;
        }

        for c in channels {
            apply_to_channel!(c, |id| {
                let handle = self.bind_channel(id, obj)?;
                if let Err(e) = self.channel_remove(handle) {
                    log::warn!("Failed to remove channel, channel has been leaked: {}", e);
                }
                Ok::<(), eyre::Report>(())
            })?;
        }

        Ok(())
    }

    // ---- Property accessors ----

    pub fn state<T: ITpProperty>(&self, state: StateHandle<T>) -> Result<&State<T>> {
        let arena = self
            .states
            .get::<StateArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(state)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn state_mut<T: ITpProperty>(&mut self, state: StateHandle<T>) -> Result<&mut State<T>> {
        let arena = self
            .states
            .get_mut::<StateArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(state)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn channel<T: ITpProperty>(&self, chan: ChannelHandle<T>) -> Result<&Channel<T>> {
        let arena = self
            .states
            .get::<ChannelArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(chan)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn channel_mut<T: ITpProperty>(
        &mut self,
        chan: ChannelHandle<T>,
    ) -> Result<&mut Channel<T>> {
        let arena = self
            .states
            .get_mut::<ChannelArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(chan)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn state_remove<T: ITpProperty>(&mut self, state: StateHandle<T>) -> Result<State<T>> {
        let arena = self
            .states
            .get_mut::<StateArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .remove(state)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn channel_remove<T: ITpProperty>(&mut self, channel: ChannelHandle<T>) -> Result<Channel<T>> {
        let arena = self
            .channels
            .get_mut::<ChannelArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .remove(channel)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    // ---- State and Channel bindings ----

    pub fn bind_state<T: ITpPropertyStatic>(
        &self,
        id: StateId<T>,
        obj: ObjectHandle,
    ) -> Result<StateHandle<T>> {
        let obj = self
            .objects
            .get(obj)
            .ok_or_else(|| eyre!("The given ObjectHandle doesn't exist in the Arena"))?;
        obj.bind_state(id)
    }

    pub fn bind_channel<T: ITpProperty>(
        &self,
        id: ChannelId<T>,
        obj: ObjectHandle,
    ) -> Result<ChannelHandle<T>> {
        let obj = self
            .objects
            .get(obj)
            .ok_or_else(|| eyre!("The given ObjectHandle doesn't exist in the Arena"))?;
        obj.bind_channel(id)
    }
}

// ---- Index traits ----

impl core::ops::Index<ObjectHandle> for Baseline {
    type Output = Object;

    fn index(&self, index: ObjectHandle) -> &Self::Output {
        &self.objects[index]
    }
}
impl core::ops::IndexMut<ObjectHandle> for Baseline {
    fn index_mut(&mut self, index: ObjectHandle) -> &mut Self::Output {
        &mut self.objects[index]
    }
}

impl<T: ITpProperty> core::ops::Index<StateHandle<T>> for Baseline {
    type Output = State<T>;

    fn index(&self, index: StateHandle<T>) -> &Self::Output {
        self.state(index).expect("Invalid handle")
    }
}
impl<T: ITpProperty> core::ops::IndexMut<StateHandle<T>> for Baseline {
    fn index_mut(&mut self, index: StateHandle<T>) -> &mut Self::Output {
        self.state_mut(index).expect("Invalid handle")
    }
}

impl<T: ITpProperty> core::ops::Index<ChannelHandle<T>> for Baseline {
    type Output = Channel<T>;

    fn index(&self, index: ChannelHandle<T>) -> &Self::Output {
        self.channel(index).expect("Invalid handle")
    }
}
impl<T: ITpProperty> core::ops::IndexMut<ChannelHandle<T>> for Baseline {
    fn index_mut(&mut self, index: ChannelHandle<T>) -> &mut Self::Output {
        self.channel_mut(index).expect("Invalid handle")
    }
}
