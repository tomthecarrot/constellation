mod dyn_data;
mod dyn_property;

pub use dyn_data::{DynTpData, TpDataType};
pub use dyn_property::{DynTpProperty, TpPropertyType};

use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use paste::paste;
use std::fmt::Debug;

// ---- ITpData and primitives ----

/// Any supported primitive type that can be stored in a property.
pub trait ITpData: 'static + Send + Sync + Debug + PartialEq + Clone + private::Sealed {
    const DATA_TYPE: TpDataType;
}

macro_rules! impl_itpdata {
    // base case
    ($t:ty) => {
        paste! {
            impl ITpData for $t {
                const DATA_TYPE: TpDataType = TpDataType::[<$t:camel>];
            }
        }

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
    ContractDataHandle,
);

// ---- ITpProperty and containers ----

pub trait ITpProperty: 'static + Send + Sync + Debug + PartialEq + Clone {
    type Data: ITpData;

    const PROPERTY_TYPE: TpPropertyType;
}

/// Vecs of ITpDatas are valid for storing in a property
impl<T: ITpData> ITpProperty for Vec<T> {
    type Data = T;

    const PROPERTY_TYPE: TpPropertyType = TpPropertyType::Vec(T::DATA_TYPE);
}

/// All ITpDatas are also valid for storing in a property
impl<T: ITpData> ITpProperty for T {
    type Data = T;

    const PROPERTY_TYPE: TpPropertyType = TpPropertyType::Single(T::DATA_TYPE);
}

mod private {
    /// Prevents trait implementation by third parties. See
    /// https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait Sealed {}
}
