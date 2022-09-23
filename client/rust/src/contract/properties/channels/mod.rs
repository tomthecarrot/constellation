mod channel;
pub mod dyn_channel;
pub mod dyn_handle;
mod handle;

mod misc;

pub use self::channel::{Channel, Keyframe};
pub use self::dyn_channel::{apply_to_channel, apply_to_channel_mut, apply_to_channel_ref};
pub use self::dyn_channel::{DynChannel, DynChannelMut, DynChannelRef};
pub use self::dyn_handle::DynChannelHandle;
pub use self::handle::{ChannelHandle, IChannelHandle};
pub use crate::contract::properties::dynamic::apply_to_channel_id;

use crate::contract::properties::dynamic::TpPropertyType;
use crate::contract::properties::dynamic::__macro::{DynEnum, DynTpPropId};
use crate::contract::properties::traits::{ITpData, ITpProperty, ITpPropertyStatic};
use crate::contract::ContractDataHandle;

use std::any::TypeId;
use std::marker::PhantomData;
use typemap::ShareMap;

/// A `TypeMap` key to access the arena containing `State<T>`s.
pub(crate) struct ChannelArenaHandle<T: ITpPropertyStatic>(PhantomData<T>);
impl<T: ITpPropertyStatic> typemap::Key for ChannelArenaHandle<T> {
    type Value = arena::Arena<Channel<T>>;
}

pub struct ChannelArenaMap(pub ShareMap);
impl ChannelArenaMap {
    pub fn new() -> Self {
        Self(ShareMap::custom())
    }

    pub fn get<T: ITpPropertyStatic>(&self) -> Option<&arena::Arena<Channel<T>>> {
        self.0.get::<ChannelArenaHandle<T>>()
    }

    pub fn get_mut<T: ITpPropertyStatic>(&mut self) -> Option<&mut arena::Arena<Channel<T>>> {
        self.0.get_mut::<ChannelArenaHandle<T>>()
    }
}

/// Represents a particular channel field of a contract. For actual channel data
/// of a specific object, see [`ChannelHandle`].
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ChannelId<T: ITpProperty> {
    idx: usize, // idx into an object's channel properties
    contract: ContractDataHandle,
    _phantom: PhantomData<T>,
}
impl<T: ITpProperty> ChannelId<T> {
    pub fn contract(&self) -> ContractDataHandle {
        self.contract
    }

    pub(crate) fn idx(&self) -> usize {
        self.idx
    }

    /// This is only exposed so that contract macros can call it. It isn't intended
    /// for direct use by API clients
    #[doc(hidden)]
    pub fn new(idx: usize, contract: ContractDataHandle) -> Self {
        Self {
            idx,
            contract,
            _phantom: PhantomData,
        }
    }
}
impl<T: ITpData> Copy for ChannelId<T> {}

pub trait IChannels {
    fn type_ids() -> &'static [TypeId];
    fn enumerate_types() -> &'static [TpPropertyType];
    fn field_names() -> &'static [&'static str];
}

impl IChannels for () {
    fn type_ids() -> &'static [TypeId] {
        &[]
    }

    fn enumerate_types() -> &'static [TpPropertyType] {
        &[]
    }

    fn field_names() -> &'static [&'static str] {
        &[]
    }
}

DynTpPropId!(DynChannelId, ChannelId);

super::prop_iter!(ChannelsIter, IChannels, DynChannelId);

#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]
    use super::*;

    pub use super::channel::c_api::*;
    pub use super::handle::c_api::*;

    use crate::contract::c_api::ContractDataHandle as CContractDataHandle;
    use crate::contract::properties::primitives;
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;

    use derive_more::{From, Into};
    use ref_cast::RefCast;
    use rsharp::remangle;
    use safer_ffi::prelude::*;

    macro_rules! monomorphize {
        // Base case
        ($path:literal, $t:ty $(,)?) => {
            paste::paste! {
                mod [<_ChannelId_ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, Into, RefCast, Copy, Clone)]
                    #[repr(C)]
                    pub struct [<ChannelId_ $t:camel>] {
                        pub inner: ChannelId<$t>,
                    }
                    pub use [<ChannelId_ $t:camel>] as Monomorphized;

                    #[ffi_export]
                    pub fn [<ChannelId_ $t:camel __contract>]<'a>(id: &'a Monomorphized) -> repr_c::Box<CContractDataHandle> {
                        repr_c::Box::new(CContractDataHandle::from(id.inner.contract()))
                    }
                }
                pub use [<_ChannelId_ $t:camel>]::Monomorphized as [<ChannelId_ $t:camel>];
            }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }
    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    primitives!(; types, monomorphize, "constellation::contract::properties::channels");
}
