use crate::contract::properties::{dynamic::TpPropertyType, traits::ITpProperty};

#[derive(Debug, PartialEq)]
pub struct Keyframe<T: ITpProperty> {
    value: T,
    time: f64,
}
impl<T: ITpProperty> Keyframe<T> {
    pub fn new(value: T, time: f64) -> Self {
        Self { value, time }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn time(&self) -> f64 {
        self.time
    }
}

#[derive(Debug)]
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
