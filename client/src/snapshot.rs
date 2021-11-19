// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::baseline::BaselineHandle;

use std::time::Duration;

pub struct Snapshot {
    time: Duration,
    baseline: BaselineHandle,
}

impl Snapshot {
    pub fn new(time: Duration, baseline: BaselineHandle) -> Self {
        Self { time, baseline }
    }
}

pub type SnapshotHandle = arena::Index<Snapshot>;
