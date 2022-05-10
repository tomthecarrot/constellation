use crate::contract::properties::dynamic::__macro::{DynEnum, DynTpPropId};
use crate::contract::properties::traits::ITpProperty;
use crate::contract::ContractDataHandle;

use std::marker::PhantomData;

#[cfg(feature = "safer-ffi")]
use safer_ffi::derive_ReprC;

#[cfg_attr(feature = "safer-ffi", derive_ReprC, ReprC::opaque)]
/// Represents a particular state field of a contract. For actual state data of
/// a specific object, see [`StateHandle`].
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct StateId<T: ITpProperty> {
    idx: usize, // idx into an object's state properties
    contract: ContractDataHandle,
    _phantom: PhantomData<T>,
}
impl<T: ITpProperty> StateId<T> {
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

DynTpPropId!(DynStateId, StateId);

#[cfg(feature = "c_api")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]

    use crate::contract::c_api::ContractDataHandle as CContractDataHandle;
    use crate::contract::properties::c_api::simple_primitives;
    use crate::contract::properties::states::StateId;
    use crate::contract::ContractDataHandle;
    use crate::object::ObjectHandle;

    use derive_more::{From, Into};
    use paste::paste;
    use ref_cast::RefCast;
    use rsharp::remangle;
    use safer_ffi::prelude::*;

    macro_rules! monomorphize {
        // Base case
        ($path:literal, $t:ty $(,)?) => {
            paste! {
                mod [<_StateId_ $t:camel>] {
                    use super::*;

                    #[remangle($path)]
                    #[derive_ReprC]
                    #[ReprC::opaque]
                    #[derive(From, Into, RefCast, Copy, Clone)]
                    #[repr(C)]
                    pub struct [<StateId_ $t:camel>] {
                        pub inner: StateId<$t>,
                    }
                    pub use [<StateId_ $t:camel>] as Monomorphized;

                    #[ffi_export]
                    pub fn [<StateId_ $t:camel __contract>]<'a>(id: &'a Monomorphized) -> repr_c::Box<CContractDataHandle> {
                        repr_c::Box::new(CContractDataHandle::from(id.inner.contract()))
                    }
                }
                pub use [<_StateId_ $t:camel>]::Monomorphized as [<StateId_ $t:camel>];
            }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }
    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    simple_primitives!(; types, monomorphize, "tp_client::contract::properties::states");
}
