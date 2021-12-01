mod dyn_data;
mod dyn_property;

use crate::contract::properties::{ChannelHandle, StateHandle};
use crate::contract::ContractHandle;
use crate::object::ObjectHandle;

use std::fmt::Debug;

use enum_dispatch::enum_dispatch;

pub use dyn_data::DynTpData;
pub use dyn_property::DynTpProperty;

// ---- ITpData and primitives ----

/// Any supported primitive type that can be stored in a property.
#[enum_dispatch]
pub trait ITpData:
    'static + Send + Sync + Debug + PartialEq + PartialOrd + Clone + private::Sealed
{
}

macro_rules! impl_itpdata {
    // base case
    ($t:ty) => {
        impl ITpData for $t {}
        impl private::Sealed for $t {}
    };
    // recursive case
    ($t:ty, $($tail:ty),+) => {
        impl_itpdata!($t);
        impl_itpdata!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_itpdata!($($tail),+);
    };
}

impl_itpdata!(
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
    bool,
    f32,
    f64,
    String,
    ObjectHandle,
    ContractHandle,
);

// ---- ITpProperty and containers ----

pub trait ITpProperty: 'static + Send + Sync + Debug + PartialEq + PartialOrd + Clone {
    type Data: ITpData;
}

/// Vecs of ITpDatas are valid for storing in a property
impl<T: ITpData> ITpProperty for Vec<T> {
    type Data = T;
}

/// All ITpDatas are also valid for storing in a property
impl<T: ITpData> ITpProperty for T {
    type Data = T;
}

mod private {
    /// Prevents trait implementation by third parties. See
    /// https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait Sealed {}
}
