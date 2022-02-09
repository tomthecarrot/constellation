//! Contains fundamental traits and types for properties.

use crate::contract::properties::dynamic::{
    DynTpPrimitive, DynTpProperty, DynTpVec, TpPrimitiveType, TpPropertyType,
};

use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use paste::paste;
use std::fmt::Debug;

// ---- ITpData and primitives ----

/// Any supported primitive type that can be stored in a property.
pub trait ITpData: 'static + Send + Sync + Debug + PartialEq + Clone + private::Sealed {
    const DATA_TYPE: TpPrimitiveType;
}

macro_rules! impl_itpdata {
    // base case
    ($t:ty) => {
        paste! {
            impl ITpData for $t {
                const DATA_TYPE: TpPrimitiveType = TpPrimitiveType::[<$t:camel>];
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

/// An `ITpProperty` is any type that could be stored inside a teleportal
/// property. For example, the `T` in `State<T>` or `Channel<T>`
pub trait ITpProperty: 'static + Send + Sync + Debug + PartialEq + Clone {
    fn prop_type(&self) -> TpPropertyType;
}

/// An `ITpPropertyStatic` is an [`ITpProperty`], with the additional restriction
/// that its concrete type is known at compile-time and is not dynamic.
pub trait ITpPropertyStatic: ITpProperty {
    const PROPERTY_TYPE: TpPropertyType;
}

/// Vecs of ITpDatas are valid for storing in a property
impl<T: ITpData> ITpPropertyStatic for Vec<T> {
    const PROPERTY_TYPE: TpPropertyType = TpPropertyType::Vec(T::DATA_TYPE);
}

/// All ITpDatas are also valid for storing in a property
impl<T: ITpData> ITpPropertyStatic for T {
    const PROPERTY_TYPE: TpPropertyType = TpPropertyType::Primitive(T::DATA_TYPE);
}

/// All ITpPropertyStatics are valid ITpProperties (but not the other way around)
impl<T: ITpPropertyStatic> ITpProperty for T {
    fn prop_type(&self) -> TpPropertyType {
        T::PROPERTY_TYPE
    }
}

mod private {
    /// Prevents trait implementation by third parties. See
    /// https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait Sealed {}
}
