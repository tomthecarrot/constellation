use crate::contract::properties::traits::ITpProperty;

use keyframe::Keyframe;

// TODO: figure out data in a channel
pub struct Channel<T: ITpProperty>(Vec<Keyframe<T>>);
impl<T: ITpProperty> Channel<T> {
    pub fn new(keyframes: impl Iterator<Item = Keyframe<T>>) -> Self {
        let mut keyframes: Vec<_> = keyframes.collect();
        // Sort keyframes by time
        keyframes.sort_unstable_by(|a, b| {
            a.time()
                .partial_cmp(&b.time())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Self(keyframes)
    }
}
