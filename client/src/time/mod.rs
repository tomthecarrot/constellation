use std::{fmt::Error, num::TryFromIntError};

use derive_more::From;

#[derive(From, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(u32);

impl Time {
    pub fn ticks(&self) -> u32 {
        self.0
    }
    pub fn to_seconds(&self) -> f32 {
        (self.0 as f32) / 1024.0
    }
}

#[derive(From, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealmTime(pub Time);

#[derive(From, Debug, Clone)]
pub struct ObjectTime {
    pub offset_parent: Time,
    pub offset_local: Time,
    pub scalar: i32,
    pub interval: u32,
}

impl ObjectTime {
    pub fn resolve_time(&self, parent_time: &Time) -> Time {
        // See v3 Spec: Time: ObjectTime for documentation on this formula.
        let mut parent_diff_precise: i64 =
            (parent_time.ticks() - self.offset_parent.ticks()) as i64;
        parent_diff_precise *= self.scalar as i64;
        parent_diff_precise >>= 10;
        parent_diff_precise += self.offset_local.ticks() as i64;
        parent_diff_precise %= self.interval as i64;

        // Convert from i64 -> u32
        let result: Result<u32, TryFromIntError> = parent_diff_precise.try_into();
        match result {
            Ok(value) => Time(value),
            Err(err) => {
                log::error!("ObjectTime could not resolve time: {}", err);
                Time(0)
            }
        }
    }
}
