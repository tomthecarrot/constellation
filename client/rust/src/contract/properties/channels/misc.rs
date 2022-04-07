use keyframe::num_traits::{AsPrimitive, Float, FromPrimitive};
use keyframe::CanTween;

/// Wraps any `T: Copy`, making it tweenable. Easing jumps from the starting
/// value to the target value when the blend value (`time`) hits `100%` or `1.0`.
///
/// For integer-style types, its more likely that you want to use [`IntTweenable`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JumpTweenable<T>(pub T);
impl<T> CanTween for JumpTweenable<T> {
    fn ease(from: Self, to: Self, time: impl Float) -> Self {
        if time.trunc().is_zero() {
            from
        } else {
            to
        }
    }
}

/// Makes any primitive int `T` tweenable, by tweening to the intermediary integer
/// values between the two points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntTweenable<T: keyframe::num_traits::PrimInt + AsPrimitive<f32> + FromPrimitive>(pub T);
impl<T: keyframe::num_traits::PrimInt + AsPrimitive<f32> + FromPrimitive> CanTween
    for IntTweenable<T>
{
    fn ease(mut from: Self, to: Self, time: impl Float) -> Self {
        let diff: f32 = (to.0 - from.0).as_();
        let scaled_diff: u32 = (time.to_f32().unwrap() * diff) as _;
        from.0 = from.0 + FromPrimitive::from_u32(scaled_diff).unwrap();
        from
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keyframe::ease;
    use keyframe::functions::{Linear, Step};

    #[test]
    fn test_jump_tween() {
        let t1 = JumpTweenable(-2);
        let t2 = JumpTweenable(8);

        assert_eq!(ease(Linear, t1, t2, 0.0), t1);
        assert_eq!(ease(Linear, t1, t2, 0.1), t1);
        assert_eq!(ease(Linear, t1, t2, 0.9), t1);
        assert_eq!(ease(Linear, t1, t2, 1.0), t2);

        assert_eq!(ease(Step, t1, t2, 0.0), t1);
        assert_eq!(ease(Step, t1, t2, 0.1), t1);
        assert_eq!(ease(Step, t1, t2, 0.49), t1);
        assert_eq!(ease(Step, t1, t2, 0.50), t2);
        assert_eq!(ease(Step, t1, t2, 0.9), t2);
        assert_eq!(ease(Step, t1, t2, 1.0), t2);
    }

    #[test]
    fn test_int_tween() {
        let t1 = IntTweenable(-2);
        let t2 = IntTweenable(8);

        // ---- Test Linear tween ----
        assert_eq!(ease(Linear, t1, t2, 0.0), t1);

        assert_eq!(ease(Linear, t1, t2, 0.09).0, -2);
        assert_eq!(ease(Linear, t1, t2, 0.10).0, -1);
        assert_eq!(ease(Linear, t1, t2, 0.11).0, -1);

        assert_eq!(ease(Linear, t1, t2, 0.89).0, 6);
        assert_eq!(ease(Linear, t1, t2, 0.90).0, 7);
        assert_eq!(ease(Linear, t1, t2, 0.91).0, 7);

        assert_eq!(ease(Linear, t1, t2, 1.0), t2);

        // ---- Test Step tween ----
        assert_eq!(ease(Step, t1, t2, 0.0), t1);
        assert_eq!(ease(Step, t1, t2, 0.1), t1);
        assert_eq!(ease(Step, t1, t2, 0.49), t1);
        assert_eq!(ease(Step, t1, t2, 0.50), t2);
        assert_eq!(ease(Step, t1, t2, 0.9), t2);
        assert_eq!(ease(Step, t1, t2, 1.0), t2);
    }
}
