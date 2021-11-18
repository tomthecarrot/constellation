// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::contract::properties::{
    Channel, ChannelArenaHandle, ChannelArenaMap, ChannelHandle, State, StateArenaHandle,
    StateArenaMap, StateHandle, TPData,
};
use crate::contract::{Contract, ContractHandle};
use crate::object::{Object, ObjectHandle};

use arena::Arena;
use eyre::{eyre, Result, WrapErr};
use typemap::TypeMap;

use rand::Rng;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::time::Duration;

pub struct BaselineGeneric {
    following: Option<&BaselineGeneric>,
    followers: HashSet<&BaselineGeneric>,
    time: &Duration,
    objects: Arena<Object>,
    contracts: Arena<Contract>,
    states: StateArenaMap,     // maps from T to Arena<State<T>>
    channels: ChannelArenaMap, // maps from T to Arena<Channel<T>>
}

impl PartialEq for BaselineGeneric {
    fn eq(&self, other: &BaselineGeneric) -> bool {
        return false;
    }
}
impl Eq for BaselineGeneric {}
impl Hash for BaselineGeneric {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let num: u64 = rand::thread_rng().next_u64();
        state.write_u64(num);
    }
}

impl BaselineGeneric {
    pub fn new(time: &Duration) -> Self {
        let following = None;
        let followers = HashSet::new();
        let objects = Arena::new();
        let contracts = Arena::new();
        let states = TypeMap::new();
        let channels = TypeMap::new();

        Self {
            following,
            followers,
            time,
            objects,
            contracts,
            states,
            channels,
        }
    }

    pub fn follow(&self, baseline: &BaselineGeneric) {
        self.following = Some(baseline);
        baseline.register_follower(&self);
    }

    pub fn unfollow(&self) {
        match self.following {
            Some(following) => {
                following.unregister_follower(&self);
            }
        }
    }

    pub fn register_follower(&self, follower: &BaselineGeneric) {
        self.followers.insert(follower);
    }

    pub fn unregister_follower(&self, follower: &BaselineGeneric) {
        self.followers.push(follower);
    }

    pub fn notify_dirty_state<T: TPData>(&self, state: StateHandle<T>) {}

    pub fn notify_dirty_channel<T: TPData>(&self, state: StateHandle<T>) {}

    // ---- Object and Contract Acessors ----

    pub fn iter_objects(&self) -> impl Iterator<Item = (ObjectHandle, &Object)> {
        self.objects.iter()
    }

    pub fn iter_contracts(&self) -> impl Iterator<Item = (ContractHandle, &Contract)> {
        self.contracts.iter()
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

    pub fn contract(&self, contract: ContractHandle) -> eyre::Result<&Contract> {
        self.contracts
            .get(contract)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn contract_mut(&mut self, contract: ContractHandle) -> eyre::Result<&mut Contract> {
        self.contracts
            .get_mut(contract)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    // ---- Property accessors ----

    pub fn state<T: TPData>(&self, state: StateHandle<T>) -> Result<&State<T>> {
        let arena = self
            .states
            .get::<StateArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(state)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn state_mut<T: TPData>(&mut self, state: StateHandle<T>) -> Result<&mut State<T>> {
        let arena = self
            .states
            .get_mut::<StateArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(state)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn channel<T: TPData>(&self, chan: ChannelHandle<T>) -> Result<&Channel<T>> {
        let arena = self
            .states
            .get::<ChannelArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(chan)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn channel_mut<T: TPData>(&mut self, chan: ChannelHandle<T>) -> Result<&mut Channel<T>> {
        let arena = self
            .states
            .get_mut::<ChannelArenaHandle<T>>()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(chan)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }
}

// ---- Index traits ----

impl core::ops::Index<ObjectHandle> for BaselineGeneric {
    type Output = Object;

    fn index(&self, index: ObjectHandle) -> &Self::Output {
        &self.objects[index]
    }
}
impl core::ops::IndexMut<ObjectHandle> for BaselineGeneric {
    fn index_mut(&mut self, index: ObjectHandle) -> &mut Self::Output {
        &mut self.objects[index]
    }
}

impl core::ops::Index<ContractHandle> for BaselineGeneric {
    type Output = Contract;

    fn index(&self, index: ContractHandle) -> &Self::Output {
        &self.contracts[index]
    }
}
impl core::ops::IndexMut<ContractHandle> for BaselineGeneric {
    fn index_mut(&mut self, index: ContractHandle) -> &mut Self::Output {
        &mut self.contracts[index]
    }
}

impl<'a, T: TPData> core::ops::Index<StateHandle<T>> for BaselineGeneric {
    type Output = State<T>;

    fn index(&self, index: StateHandle<T>) -> &Self::Output {
        self.state(index).expect("Invalid handle")
    }
}
impl<'a, T: TPData> core::ops::IndexMut<StateHandle<T>> for BaselineGeneric {
    fn index_mut(&mut self, index: StateHandle<T>) -> &mut Self::Output {
        self.state_mut(index).expect("Invalid handle")
    }
}

impl<'a, T: TPData> core::ops::Index<ChannelHandle<T>> for BaselineGeneric {
    type Output = Channel<T>;

    fn index(&self, index: ChannelHandle<T>) -> &Self::Output {
        self.channel(index).expect("Invalid handle")
    }
}
impl<'a, T: TPData> core::ops::IndexMut<ChannelHandle<T>> for BaselineGeneric {
    fn index_mut(&mut self, index: ChannelHandle<T>) -> &mut Self::Output {
        self.channel_mut(index).expect("Invalid handle")
    }
}
