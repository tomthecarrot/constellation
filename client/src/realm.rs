// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::baseline::{Baseline, BaselineHandle};

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
    baselines: Arena<Baseline>,
    baseline_main: BaselineHandle,
    baseline_fork: BaselineHandle,
}
impl Realm {
    pub fn new(realm_id: RealmID) -> Self {
        // Initialize time and arena allocators.
        let time = Duration::ZERO;
        let mut baselines = Arena::new();

        // Create the BaselineFork.
        let baseline_fork = Baseline::new(None);
        let baseline_fork_handle: BaselineHandle = baselines.insert(baseline_fork);

        // Create the BaselineMain, registering the fork.
        let baseline_main = Baseline::new(Some(baseline_fork_handle));
        let baseline_main_handle: BaselineHandle = baselines.insert(baseline_main);

        Self {
            realm_id: realm_id,
            time: time,
            baselines: baselines,
            baseline_main: baseline_main_handle,
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

    pub fn baseline_main(&self) -> BaselineHandle {
        self.baseline_main
    }

    pub fn baseline_fork(&self) -> BaselineHandle {
        self.baseline_fork
    }
}
