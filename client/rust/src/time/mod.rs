use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use derive_more::From;

/// Time is represented in ticks, with the resolution provided by `ticks_per_second()`.
#[derive(Debug, Default, Clone, Copy, From, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticks(i32);

impl Ticks {
    pub const fn ticks_per_second() -> i32 {
        return 1000;
    }

    pub fn new(millis: i32) -> Self {
        Self(millis)
    }

    pub fn as_millis(&self) -> i32 {
        self.0
    }

    pub fn as_millis_i64(&self) -> i64 {
        self.0 as i64
    }

    /// The return value of this function truncates the actual `seconds` value.
    /// Use `Time.as_secs_f32()` for more precision.
    pub fn as_secs(&self) -> i32 {
        self.0 >> 10 // normalize scalar factor (1024 denominator)
    }

    pub fn as_secs_f32(&self) -> f32 {
        (self.0 as f32) / 1024.0
    }
}

impl Add for Ticks {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0.saturating_add(other.0))
    }
}

impl Sub for Ticks {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.saturating_sub(other.0))
    }
}

impl Mul<TimeScale> for Ticks {
    type Output = Self;

    fn mul(self, other: TimeScale) -> Self::Output {
        let mut val = self.0 as i64;
        val *= other.numerator_as_i64();
        val >>= TimeScale::denominator_as_log2();
        Self(val as i32)
    }
}

impl AddAssign for Ticks {
    fn add_assign(&mut self, other: Self) {
        *self = Self::add(*self, other);
    }
}

impl SubAssign for Ticks {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::sub(*self, other);
    }
}

impl MulAssign<TimeScale> for Ticks {
    fn mul_assign(&mut self, other: TimeScale) {
        *self = Self::mul(*self, other);
    }
}

/// Fixed-point value with a denominator as defined by [`TimeScale::denominator()`].
/// For a denominator of 1024: 512 = 0.5x speed, 1024 = 1x speed, 2048 = 2x speed, etc.
#[derive(Debug, Clone, Copy, From, PartialEq, Eq)]
pub struct TimeScale(i32);

impl TimeScale {
    pub fn numerator(&self) -> i32 {
        self.0
    }
    pub fn numerator_as_i64(&self) -> i64 {
        self.numerator() as i64
    }
    pub const fn denominator_as_log2() -> u8 {
        10
    }
    pub const fn denominator() -> u16 {
        1 << TimeScale::denominator_as_log2()
    }
    pub fn as_f32(&self) -> f32 {
        (self.numerator() as f32) / (Self::denominator() as f32)
    }
    pub fn as_f64(&self) -> f64 {
        (self.numerator() as f64) / (Self::denominator() as f64)
    }
}

impl Default for TimeScale {
    fn default() -> TimeScale {
        TimeScale(TimeScale::denominator().into())
    }
}

/// Local time within a channel, measured in ticks and always starting at 0.
#[derive(From, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelTime(Ticks);

impl ChannelTime {
    pub fn ticks(&self) -> Ticks {
        self.0
    }
    pub fn warp(&self, warp: &TimeWarp) -> ChannelTime {
        warp.apply(*self)
    }
}

/// Represents the amount of time (in ticks) that has passed since the current
/// RealmSession was started. Passes at the same speed as wall time.
#[derive(From, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RealmTime(Ticks);

impl RealmTime {
    pub fn ticks(&self) -> Ticks {
        self.0
    }
    pub fn ticks_mut(&mut self) -> &mut Ticks {
        &mut self.0
    }
}

/// Contains information for how an [`Object`]'s `ChannelTime`s relate to the [`Object`]'s parent
/// in the Realm Object hierarchy, and provides a means of calculating the resulting `ChannelTime`s.
#[derive(Debug, Default, From, Clone)]
pub struct TimeWarp {
    /// A start time (in `Ticks`) for this [`Object`]'s Channel sampling relative to the parent's time.
    /// For example, an offset of `3 * Ticks::ticks_per_second()` will cause the time to shift to 3 seconds
    /// after its parent's time begins ticking from 0.
    pub offset: Ticks,

    pub scale: TimeScale,
}

impl TimeWarp {
    /// Calculates the `ChannelTime` for this `Object` relative to a given parent `ChannelTime`.
    pub fn apply(&self, parent_time: ChannelTime) -> ChannelTime {
        // See v3 Spec: Time: ObjectTime for documentation on this formula.
        ChannelTime((parent_time.ticks() - self.offset) * self.scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DENOMINATOR: i32 = TimeScale::denominator() as i32;

    #[test]
    fn test_resolve_time() {
        let parent_time = ChannelTime(Ticks(4096));

        let warp_1 = TimeWarp {
            offset: Ticks(2048),
            scale: TimeScale(DENOMINATOR * 2),
        };
        let correct_value_1 = 4096;

        let result_1 = parent_time.warp(&warp_1);
        assert_eq!(result_1.ticks().as_millis(), correct_value_1);

        let warp_2 = TimeWarp {
            offset: Ticks(5000),
            scale: TimeScale(DENOMINATOR / -2),
        };
        let correct_value_2 = 452;

        let result_2 = parent_time.warp(&warp_2);
        assert_eq!(result_2.ticks().as_millis(), correct_value_2);

        let warp_3 = TimeWarp {
            offset: Ticks(0),
            scale: TimeScale(DENOMINATOR / 2),
        };
        let correct_value_3 = 2048;

        let result_3 = parent_time.warp(&warp_3);
        assert_eq!(result_3.ticks().as_millis(), correct_value_3);
    }
}
