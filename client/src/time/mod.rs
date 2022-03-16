use std::num::TryFromIntError;

use derive_more::From;

/// Time is represented in milliseconds (ms).
#[derive(From, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(u32);

impl Time {
    pub fn as_millis(&self) -> u32 {
        self.0
    }

    /// The return value of this function truncates the actual `seconds` value.
    /// Use `Time.as_secs_f32()` for more precision.
    pub fn as_secs(&self) -> u32 {
        self.0 >> 10 // normalize scalar factor (1024 denominator)
    }

    pub fn as_secs_f32(&self) -> f32 {
        (self.0 as f32) / 1024.0
    }
}

#[derive(From, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealmTime(pub Time);

#[derive(From, Debug, Clone)]
pub struct ObjectTime {
    /// A start time (in milliseconds) for this Object's Channel sampling relative to the parent's time.
    /// For example, with solely a parent offset of 3000, this Object will begin playing 3 seconds
    /// after its parent's time begins ticking from 0. This is affected by `scalar`.
    pub offset_parent: Time,

    /// A final value (in milliseconds) that is added to the scaled parent value.
    pub offset_local: Time,

    /// A fixed point value with a 1024 denominator.
    /// 512 = 0.5x speed, 1024 = 1x speed, 2048 = 2x speed, etc.
    pub scalar: i32,

    /// An optional modulo applied to this `ObjectTime` after `offset_local` is applied.
    pub interval: Option<u16>,
}

impl Default for ObjectTime {
    fn default() -> ObjectTime {
        ObjectTime {
            offset_parent: Time(0),
            offset_local: Time(0),
            scalar: 1,
            interval: None,
        }
    }
}

impl ObjectTime {
    /// Calculates this `ObjectTime` relative to a given parent time, which can be either
    /// `RealmTime` or `ObjectTime`.
    pub fn resolve_time(&self, parent_time: &Time) -> Result<Time, TryFromIntError> {
        // See v3 Spec: Time: ObjectTime for documentation on this formula.
        let mut parent_diff_precise: i64 =
            (parent_time.as_millis() - self.offset_parent.as_millis()) as i64;

        // Multiply and normalize scalar (1024 denominator)..
        parent_diff_precise *= self.scalar as i64;
        parent_diff_precise >>= 10;

        // Add local offset.
        parent_diff_precise += self.offset_local.as_millis() as i64;

        // Apply modulo if applicable.
        if let Some(value) = self.interval {
            parent_diff_precise %= value as i64;
        }

        // Convert from i64 -> u32.
        let result: Result<u32, TryFromIntError> = parent_diff_precise.try_into();
        match result {
            Ok(value) => Ok(Time(value)),
            Err(err) => Err(err),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_resolve_time() {
        let parent_time = Time(4096);

        let object_time_1 = ObjectTime {
            offset_parent: Time(0),
            offset_local: Time(2048),
            scalar: 1024,
            interval: None,
        };
        let resolved_time_1 = object_time_1.resolve_time(&parent_time);
        match resolved_time_1 {
            Ok(value) => {
                assert_eq!(value.as_millis(), 6144);
            }
            Err(err) => {}
        }

        let object_time_2 = ObjectTime {
            offset_parent: Time(2048),
            offset_local: Time(512),
            scalar: 512,
            interval: None,
        };
        let resolved_time_2 = object_time_2.resolve_time(&parent_time);
        match resolved_time_2 {
            Ok(value) => {
                assert_eq!(value.as_millis(), 1536);
            }
            Err(err) => {}
        }

        let object_time_3 = ObjectTime {
            offset_parent: Time(2048),
            offset_local: Time(200),
            scalar: 0,
            interval: None,
        };
        let resolved_time_3 = object_time_3.resolve_time(&parent_time);
        match resolved_time_3 {
            Ok(value) => {
                assert_eq!(value.as_millis(), 200);
            }
            Err(err) => {}
        }

        let object_time_4 = ObjectTime {
            offset_parent: Time(2048),
            offset_local: Time(200),
            scalar: 0,
            interval: Some(3),
        };
        let resolved_time_4 = object_time_4.resolve_time(&parent_time);
        match resolved_time_4 {
            Ok(value) => {
                assert_eq!(value.as_millis(), 2);
            }
            Err(err) => {}
        }
    }
}
