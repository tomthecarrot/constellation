use super::{DynChannelMut, DynChannelRef};
use crate::contract::properties::channels::Channel;
use crate::contract::properties::primitives;
use crate::contract::ContractDataHandle;
use crate::contract::ObjectHandle;

macro_rules! impl_from {
    // base case
    ($t:ty) => {
        impl <'a> From<&'a Channel<$t>> for DynChannelRef<'a> {
            fn from(other: &'a Channel<$t>) -> Self {
                Self::Primitive(other.into())
            }
        }

        impl <'a> From<&'a mut Channel<$t>> for DynChannelMut<'a> {
            fn from(other: &'a mut Channel<$t>) -> Self {
                Self::Primitive(other.into())
            }
        }

        impl <'a> From<&'a Channel<Vec<$t>>> for DynChannelRef<'a> {
            fn from(other: &'a Channel<Vec<$t>>) -> Self {
                Self::Vec(other.into())
            }
        }

        impl <'a> From<&'a mut Channel<Vec<$t>>> for DynChannelMut<'a> {
            fn from(other: &'a mut Channel<Vec<$t>>) -> Self {
                Self::Vec(other.into())
            }
        }
    };

    // recursive case
    ($t:ty, $($tail:ty),+ $(,)?) => {
        impl_from!($t);
        impl_from!($($tail),+);
    };
}

primitives!(; types, impl_from);
