use crate::contract::{Contract, ContractID};
use crate::object::{Object, ObjectID};
use crate::properties::{Channel, State};

use arena::Arena;

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
    states: Arena<State>,
    channels: Arena<Channel>,
    time: std::time::Duration,
}
impl Realm {
    pub fn new(realm_id: RealmID) -> Self {
        let objects = Arena::new();
        let time = std::time::Duration::ZERO;
        let contracts = Arena::new();
        let states = Arena::new();
        let channels = Arena::new();
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

    pub fn iter_objects(&self) -> impl Iterator<Item = (ObjectID, &Object)> {
        self.objects.iter()
    }

    pub fn iter_contracts(&self) -> impl Iterator<Item = (ContractID, &Contract)> {
        self.contracts.iter()
    }
}
impl core::ops::Index<ObjectID> for Realm {
    type Output = Object;

    fn index(&self, index: ObjectID) -> &Self::Output {
        &self.objects[index]
    }
}
impl core::ops::IndexMut<ObjectID> for Realm {
    fn index_mut(&mut self, index: ObjectID) -> &mut Self::Output {
        todo!()
    }
}
