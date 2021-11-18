// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::baseline::{BaselineGeneric, BaselineGenericHandle};
use crate::contract::properties::{
    Channel, ChannelArenaHandle, ChannelArenaMap, ChannelHandle, State, StateArenaHandle,
    StateArenaMap, StateHandle, TPData,
};
use crate::contract::{Contract, ContractHandle};
use crate::object::{Object, ObjectHandle};
use crate::snapshot::{Snapshot, SnapshotHandle};

use arena::Arena;

use std::time::Duration;

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
    time: Duration,
    snapshots: Arena<Snapshot>,
    baselines_generic: Arena<BaselineGeneric>,
    baseline: BaselineGenericHandle,
    baseline_fork: BaselineGenericHandle,
}
impl Realm {
    pub fn new(realm_id: RealmID) -> Self {
        let time = Duration::ZERO;
        let snapshots = Arena::new();
        let baselines_generic = Arena::new();

        let baseline = BaselineGeneric::new(&time);
        let baseline_fork = BaselineGeneric::new(&time);

        let baseline_handle = baselines_generic.insert(baseline);
        let baseline_fork_handle = baselines_generic.insert(baseline_fork);
        baseline.set_handle(baseline_handle);
        baseline_fork.set_handle(baseline_fork_handle);

        baseline_fork.follow(baseline_handle);

        Self {
            realm_id: realm_id,
            time: time,
            snapshots: snapshots,
            baselines_generic: baselines_generic,
            baseline: baseline_handle,
            baseline_fork: baseline_fork_handle,
        }
    }

    pub fn id(&self) -> &RealmID {
        &self.realm_id
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }

    // ---- Baseline Accessors ----

    pub fn baseline(&self) -> BaselineGenericHandle {
        self.baseline
    }

    pub fn baseline_fork(&self) -> BaselineGenericHandle {
        self.baseline_fork
    }

    // ---- BaselineFork / Snapshot ----

    pub fn take_snapshot(&self) -> SnapshotHandle {
        let snapshot_baseline = BaselineGeneric::new(&self.time);
        snapshot_baseline.follow(self.baseline_fork);

        let snapshot_baseline_handle = self.baselines_generic.insert(snapshot_baseline);
        snapshot_baseline.set_handle(snapshot_baseline_handle);

        let snapshot = Snapshot::new(self.time, snapshot_baseline_handle);
        let snapshot_handle = self.snapshots.insert(snapshot);

        snapshot_handle
    }

    pub fn get_snapshot(&self, handle: SnapshotHandle) -> Option<&Snapshot> {
        self.snapshots.get(handle)
    }
}
