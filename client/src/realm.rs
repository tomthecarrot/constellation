// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::baseline::{BaselineGeneric, BaselineGenericHandle};
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
        // Initialize time and arena allocators.
        let time = Duration::ZERO;
        let snapshots = Arena::new();
        let mut baselines_generic = Arena::new();

        // Create the Baseline and BaselineFork.
        let baseline = BaselineGeneric::new();
        let baseline_fork = BaselineGeneric::new();
        let baseline_handle: BaselineGenericHandle = baselines_generic.insert(baseline);
        let baseline_fork_handle: BaselineGenericHandle = baselines_generic.insert(baseline_fork);

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

    pub fn baseline_follow(
        &mut self,
        enabled: bool,
        follower_handle: BaselineGenericHandle,
        target_handle: BaselineGenericHandle,
    ) {
        // Tell the new follower that it is now following the target.
        let follower_option = self.baselines_generic.get_mut(follower_handle);
        match follower_option {
            Some(follower) => {
                if enabled {
                    follower.start_following(target_handle);
                } else {
                    follower.stop_following();
                }
            }
            None => {
                eprintln!(
                    "[Realm] Cannot follow/unfollow: `follower` does not exist in baselines."
                );
            }
        }

        // Register/unregister the new follower with the target.
        let target_option = self.baselines_generic.get_mut(target_handle);
        match target_option {
            Some(target) => {
                if enabled {
                    target.register_follower(follower_handle);
                } else {
                    target.unregister_follower(follower_handle);
                }
            }
            None => {
                eprintln!("[Realm] Cannot follow/unfollow: `target` does not exist in baselines.");
            }
        }
    }

    // ---- BaselineFork / Snapshot ----

    pub fn take_snapshot(&mut self) -> SnapshotHandle {
        // Create a baseline for this snapshot. Store it in baselines.
        let snapshot_baseline = BaselineGeneric::new();
        let snapshot_baseline_handle: BaselineGenericHandle =
            self.baselines_generic.insert(snapshot_baseline);

        // Create a snapshot containing the baseline handle.
        let snapshot = Snapshot::new(self.time, snapshot_baseline_handle);
        let snapshot_handle: SnapshotHandle = self.snapshots.insert(snapshot);

        // Connect this snapshot to the current baseline fork.
        self.baseline_follow(true, snapshot_baseline_handle, self.baseline_fork);

        snapshot_handle
    }

    pub fn get_snapshot(&self, handle: SnapshotHandle) -> Option<&Snapshot> {
        self.snapshots.get(handle)
    }
}
