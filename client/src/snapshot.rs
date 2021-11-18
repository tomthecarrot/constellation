// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::baseline::BaselineGenericHandle;

use std::time::Duration;

pub struct Snapshot {
    time: Duration,
    baseline: BaselineGenericHandle,
}

impl Snapshot {
    pub fn new(time: Duration, baseline: BaselineGenericHandle) -> Self {
        Self { time, baseline }
    }
}

pub type SnapshotHandle = arena::Index<Snapshot>;
