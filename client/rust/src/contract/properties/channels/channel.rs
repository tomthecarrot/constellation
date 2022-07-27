use crate::contract::properties::traits::ITpProperty;

#[cfg_attr(feature = "c_api", safer_ffi::derive_ReprC, ReprC::opaque)]
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
    use crate::contract::properties::c_api::{c_types, copy_primitives, impl_from_refcast};
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;

    use derive_more::{From, Into};
    use ref_cast::RefCast;
    use rsharp::{remangle, rvec_fns};
    use safer_ffi::prelude::*;

    macro_rules! monomorphize {
        // Base case
        ($path:literal, $t:ty $(,)?) => {
            paste::paste! {
                // Module is simply to prevent name collisions on the rust side. It does
                // nothing for C
                mod [<_Keyframe_ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, Into, RefCast)]
                    #[repr(C)]
                    pub struct [<Keyframe_ $t:camel>]{
                        pub inner: Keyframe<$t>
                    }
                    pub use [<Keyframe_ $t:camel>] as Monomorphized;
                    impl_from_refcast!(Keyframe<$t>, Monomorphized);

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe_ $t:camel __new>](value: repr_c::Box<c_types::$t>, time: f64) -> repr_c::Box<Monomorphized> {
                        let value = $t::from(*value);
                        repr_c::Box::new(Keyframe::new(value, time).into())
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe_ $t:camel __drop>](kf: repr_c::Box<Monomorphized>) {
                        drop(kf)
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe_ $t:camel __value>]<'a>(kf: &'a Monomorphized) -> &'a c_types::$t {
                        kf.inner.value().into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Keyframe_ $t:camel __time>](kf: &Monomorphized) -> f64 {
                        kf.inner.time()
                    }

                    rvec_fns!($path, [<Keyframe_ $t:camel>]);
                }

                mod [<_Channel_ $t:camel>] {
                    use super::*;

                    // TODO(SER-362)
                    use [<_Keyframe_ $t:camel>]::Monomorphized as Keyframe_Monomorphized;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, Into, RefCast)]
                    #[repr(C)]
                    pub struct [<Channel_ $t:camel>]{
                        pub inner: Channel<$t>
                    }
                    pub use [<Channel_ $t:camel>] as Monomorphized;
                    impl_from_refcast!(Channel<$t>, Monomorphized);

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Channel_ $t:camel __new>](mut v: repr_c::Vec<Keyframe_Monomorphized>) -> repr_c::Box<Monomorphized> {
                        let v = v.with_rust_mut(|v| {
                            let mut tmp = std::vec::Vec::new();
                            std::mem::swap(v, &mut tmp);
                            tmp
                        });
                        let c = Channel::new(v.into_iter().map(|item| item.inner));
                        Box::new(Monomorphized::from(c)).into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Channel_ $t:camel __drop>](c:repr_c::Box<Monomorphized>) {
                        drop(c)
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Channel_ $t:camel __keyframes>]<'a>(chan: &'a Monomorphized) -> repr_c::Vec<&'a Keyframe_Monomorphized> {
                        // TODO(SER-365): Avoid allocation
                        let v: Vec<&'a Keyframe_Monomorphized> = chan.inner.keyframes().iter().map(|k| k.into()).collect();
                        v.into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Channel_ $t:camel __keyframes_mut>]<'a>(chan: &'a mut Monomorphized) -> repr_c::Vec<&'a mut Keyframe_Monomorphized> {
                        let v: Vec<&'a mut Keyframe_Monomorphized> = chan.inner.keyframes_mut().iter_mut().map(|k| k.into()).collect();
                        v.into()
                    }
                }

                pub use [<_Keyframe_ $t:camel>]::Monomorphized as [<Keyframe_ $t:camel>];
                pub use [<_Channel_ $t:camel>]::Monomorphized as [<Channel_ $t:camel>];
            }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }

    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    copy_primitives!(; types, monomorphize, "tp_client::contract::properties::channels");
}
