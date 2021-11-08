use crate::contract::properties::{Channel, ChannelArenaMap, State, StateArenaMap};
use crate::contract::{Contract, ContractHandle};
use crate::object::{Object, ObjectHandle};
use arena::Arena;

use typemap::TypeMap;

pub struct RealmID(String);
impl RealmID {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// A Realm holds all the data necessary to describe the state of a particular
/// virtual space. This includes but is not limited to contracts, objects, and
/// additional data global to that virtual space.
pub struct Realm {
    realm_id: RealmID,
    objects: Arena<Object>,
    contracts: Arena<Contract>,
    states: StateArenaMap,     // maps from T to Arena<State<T>>
    channels: ChannelArenaMap, // maps from T to Arena<Channel<T>>
    time: std::time::Duration,
}
impl Realm {
    pub fn new(realm_id: RealmID) -> Self {
        let objects = Arena::new();
        let time = std::time::Duration::ZERO;
        let contracts = Arena::new();
        let states = TypeMap::new();
        let channels = TypeMap::new();
        Self {
            realm_id,
            objects,
            time,
            contracts,
            states,
            channels,
        }
    }

    pub fn id(&self) -> &RealmID {
        &self.realm_id
    }

    pub fn time(&self) -> std::time::Duration {
        self.time
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = (ObjectHandle, &Object)> {
        self.objects.iter()
    }

    pub fn iter_contracts(&self) -> impl Iterator<Item = (ContractHandle, &Contract)> {
        self.contracts.iter()
    }
}
impl core::ops::Index<ObjectHandle> for Realm {
    type Output = Object;

    fn index(&self, index: ObjectHandle) -> &Self::Output {
        &self.objects[index]
    }
}
impl core::ops::IndexMut<ObjectHandle> for Realm {
    fn index_mut(&mut self, index: ObjectHandle) -> &mut Self::Output {
        todo!()
    }
}
