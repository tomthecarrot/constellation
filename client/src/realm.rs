// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::baseline::{Baseline, BaselineKind};

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
    baseline_main: Baseline,
    baseline_fork: Baseline,
}
impl Realm {
    pub fn new(realm_id: RealmID) -> Self {
        // Initialize time and arena allocators.
        let time = Duration::ZERO;

        // Create the BaselineFork.
        let baseline_fork = Baseline::new(BaselineKind::Fork);

        // Create the BaselineMain, registering the fork.
        let baseline_main = Baseline::new(BaselineKind::Main);

        Self {
            realm_id,
            time,
            baseline_main,
            baseline_fork,
        }
    }

    pub fn id(&self) -> &RealmID {
        &self.realm_id
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }

    // ---- Baseline Accessors ----

    pub fn baseline(&self, kind: BaselineKind) -> &Baseline {
        match kind {
            BaselineKind::Main => &self.baseline_main,
            BaselineKind::Fork => &self.baseline_fork,
        }
    }

    pub fn baseline_mut(&mut self, kind: BaselineKind) -> &mut Baseline {
        match kind {
            BaselineKind::Main => &mut self.baseline_main,
            BaselineKind::Fork => &mut self.baseline_fork,
        }
    }
}
