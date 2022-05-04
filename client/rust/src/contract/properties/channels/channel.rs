use crate::contract::properties::traits::ITpProperty;

#[cfg(feature = "safer-ffi")]
use ::safer_ffi::derive_ReprC;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "safer-ffi", derive_ReprC, ReprC::opaque)]
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

    pub fn keyframes(&self) -> &Vec<Keyframe<T>> {
        &self.0
    }

    pub fn keyframes_mut(&mut self) -> &mut Vec<Keyframe<T>> {
        &mut self.0
    }
}

#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]

    use super::*;
    use crate::contract::properties::c_api::simple_primitives;
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;

    use derive_more::From;
    use ref_cast::RefCast;
    use rsharp::remangle;
    use safer_ffi::prelude::*;

    macro_rules! monomorphize {
        // Base case
        ($path:literal, $t:ty $(,)?) => {
            paste::paste! {
                // Module is simply to prevent name collisions on the rust side. It does
                // nothing for C
                mod [<_Keyframe _ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, RefCast)]
                    #[repr(C)]
                    pub struct [<Keyframe _ $t:camel>]{
                        pub inner: Keyframe<$t>
                    }
                    use [<Keyframe _ $t:camel>] as Monomorphized;

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe _ $t:camel __new>](value: repr_c::Box<$t>, time: f64) -> repr_c::Box<Monomorphized> {
                        let value = *(value.into());
                        repr_c::Box::new(Keyframe::new(value, time).into())
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe _ $t:camel __drop>](kf: repr_c::Box<Monomorphized>) {
                        drop(kf)
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe _ $t:camel __value>]<'a>(kf: &'a Monomorphized) -> &'a $t {
                        kf.inner.value()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe _ $t:camel __time>](kf: &Monomorphized) -> f64 {
                        kf.inner.time()
                    }
                }

                mod [<_Channel _ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, RefCast)]
                    #[repr(C)]
                    pub struct [<Channel _ $t:camel>]{
                        pub inner: Channel<$t>
                    }

                    use [<Channel _ $t:camel>] as Monomorphized;

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Channel _ $t:camel __keyframes>]<'a>(chan: &'a Monomorphized) -> c_slice::Ref<'a, Keyframe<$t>> {
                        chan.inner.keyframes().as_slice().into()
                    }

                    pub fn [<Channel _ $t:camel __keyframes_mut>]<'a>(chan: &'a mut Monomorphized) -> c_slice::Mut<'a, Keyframe<$t>> {
                        chan.inner.keyframes_mut().as_mut_slice().into()
                    }
                }
            }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }

    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    simple_primitives!(; types, monomorphize, "tp_client::contract::properties::channels");
}
