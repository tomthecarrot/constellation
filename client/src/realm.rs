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
        let time = Duration::ZERO;
        let snapshots = Arena::new();
        let baselines_generic = Arena::new();

        let baseline = BaselineGeneric::new();
        let baseline_fork = BaselineGeneric::new();

        let baseline_handle = baselines_generic.insert(baseline);
        let baseline_fork_handle = baselines_generic.insert(baseline_fork);
        baseline.set_handle(baseline_handle);
        baseline_fork.set_handle(baseline_fork_handle);

        Self {
            realm_id: realm_id,
            time: time,
            snapshots: snapshots,
            baselines_generic: baselines_generic,
            baseline: baseline_handle,
            baseline_fork: baseline_fork_handle,
        }
    }

    pub fn init(&self) {
        self.baseline_follow(self.baseline_fork, self.baseline);
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
        &self,
        follower_handle: BaselineGenericHandle,
        target_handle: BaselineGenericHandle,
    ) {
        let follower_option = self.baselines_generic.get(follower_handle);
        let target_option = self.baselines_generic.get(target_handle);

        match follower_option {
            Some(follower) => match target_option {
                Some(target) => {
                    follower.start_following(target_handle);
                    target.register_follower(follower_handle);
                }
                None => {
                    eprintln!("[Realm] Cannot follow: `target` does not exist in the baselines.");
                }
            },
            None => {
                eprintln!("[Realm] Cannot follow: `follower` does not exist in the baselines.");
            }
        }
    }

    pub fn baseline_unfollow(
        &self,
        follower_handle: BaselineGenericHandle,
        target_handle: BaselineGenericHandle,
    ) {
        let follower_option = self.baselines_generic.get(follower_handle);
        let target_option = self.baselines_generic.get(target_handle);

        match follower_option {
            Some(follower) => match target_option {
                Some(target) => {
                    follower.stop_following();
                    target.unregister_follower(follower_handle);
                }
                None => {
                    eprintln!("[Realm] Cannot follow: `target` does not exist in the baselines.");
                }
            },
            None => {
                eprintln!("[Realm] Cannot follow: `follower` does not exist in the baselines.");
            }
        }
    }

    // ---- BaselineFork / Snapshot ----

    pub fn take_snapshot(&mut self) -> SnapshotHandle {
        let snapshot_baseline = BaselineGeneric::new();
        let snapshot_baseline_ref = &mut snapshot_baseline;
        let snapshot_baseline_handle = self.baselines_generic.insert(snapshot_baseline);

        snapshot_baseline_ref.set_handle(snapshot_baseline_handle);

        // let snapshot_baseline_option = self.baselines_generic.get(snapshot_baseline_handle);
        // match snapshot_baseline_option {
        //     Some(snapshot_baseline) => {
        //         snapshot_baseline.set_handle(snapshot_baseline_handle);
        //     }
        //     None => {
        //         eprintln!("[Realm] Cannot take snapshot: ");
        //     }
        // }

        let snapshot = Snapshot::new(self.time, snapshot_baseline_handle);
        let snapshot_handle = self.snapshots.insert(snapshot);

        self.baseline_follow(snapshot_baseline_handle, self.baseline);

        snapshot_handle
    }

    pub fn get_snapshot(&self, handle: SnapshotHandle) -> Option<&Snapshot> {
        self.snapshots.get(handle)
    }
}
