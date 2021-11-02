use crate::contract::{Contract, ContractID};
use crate::object::{Object, ObjectID};

use std::collections::{HashMap, HashSet};

pub struct RealmID(String);
impl RealmID {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

pub struct Realm {
    realm_id: RealmID,
    objects: HashMap<ObjectID, Object>,
    contracts: HashMap<ContractID, Contract>,
    time: std::time::Duration,
}
impl Realm {
    pub fn new(realm_id: RealmID) -> Self {
        let objects = HashMap::default();
        let time = std::time::Duration::ZERO;
        let contracts = HashMap::default();
        Self {
            realm_id,
            objects,
            time,
            contracts,
        }
    }

    pub fn id(&self) -> &RealmID {
        &self.realm_id
    }

    pub fn time(&self) -> std::time::Duration {
        self.time
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = &ObjectID> {
        self.objects.keys()
    }

    pub fn iter_contracts(&self) -> impl Iterator<Item = &ContractID> {
        self.contracts.keys()
    }
}
