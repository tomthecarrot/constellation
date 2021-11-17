// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::contract::properties::{
    Channel, ChannelArenaHandle, ChannelArenaMap, ChannelHandle, State, StateArenaHandle,
    StateArenaMap, StateHandle, TPData,
};
use crate::contract::{Contract, ContractHandle};
use crate::object::{Object, ObjectHandle};
use crate::baseline::BaselineGeneric;

use std::time::Duration;
use std::collections::HashMap;

pub struct RealmID(String);
impl RealmID {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

/// A Realm holds all the data necessary to describe the state of a particular
/// virtual space. This includes but is not limited to contracts, objects, and
/// additional data global to that virtual space.
pub struct Realm<'a> {
    realm_id: RealmID,
    time: Duration,
    baseline: BaselineGeneric<'a>,
    baseline_fork: BaselineGeneric<'a>,
    snapshots: HashMap<Duration, BaselineGeneric<'a>>
}
impl<'a> Realm<'a> {
    pub fn new(realm_id: RealmID) -> Self {
        let time = &Duration::ZERO;
        let baseline = BaselineGeneric::new(&time);
        let baseline_fork = BaselineGeneric::new(&time);
        baseline_fork.follow(&baseline);
        let snapshots = HashMap::new();

        Self {
            realm_id,
            time: *time,
            baseline,
            baseline_fork,
            snapshots
        }
    }

    pub fn id(&self) -> &RealmID {
        &self.realm_id
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }

    // ---- Baseline Accessors ----

    pub fn baseline(&self) -> &BaselineGeneric {
        &self.baseline
    }

    pub fn baseline_fork(&self) -> &BaselineGeneric {
        &self.baseline_fork
    }

    // ---- BaselineFork / Snapshot ----

    pub fn take_snapshot(&'a mut self) {
        let snapshot = BaselineGeneric::new(&self.time);
        snapshot.follow(&self.baseline_fork);
        self.snapshots.insert(self.time, snapshot);
    }
}
