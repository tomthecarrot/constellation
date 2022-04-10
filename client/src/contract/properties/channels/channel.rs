use crate::contract::properties::traits::ITpProperty;

#[derive(Debug, PartialEq)]
#[repr(C)]
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

#[cfg(feature = "c_api")]
#[tp_rsharp::substitute("tp_client::contract::properties::channels")]
pub mod c_api {
    #![allow(non_camel_case_types, unused)]

    use super::*;
    use crate::contract::properties::primitives;
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;
    use tp_rsharp::{monomorphize, remangle};

    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    primitives!(; types, monomorphize, "tp_client::contract::properties::channels", Keyframe);

    #[no_mangle]
    pub extern "C" fn tp_client__contract__properties__channels__get_sample_keyframe_u8(
    ) -> tp_client__contract__properties__channels__Keyframe_U8 {
        tp_client__contract__properties__channels__Keyframe_U8(Keyframe::new(1, 0.0))
    }
}
