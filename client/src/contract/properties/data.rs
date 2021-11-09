use crate::contract::properties::{ChannelHandle, StateHandle};
use crate::contract::ContractHandle;
use crate::object::ObjectHandle;

/// Any data that can be stored in a property implements `TPData`.
pub trait TPData: Sized + 'static + private::Sealed {}

// ---- Trait implementations for funamental datatypes

mod private {
    /// Prevents trait implementation by third parties. See
    /// https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait Sealed {}
}

macro_rules! impl_macro {
    // base case
    ($t:ty) => {
        impl TPData for $t {}
        impl private::Sealed for $t {}
    };
    // recursive case
    ($t:ty, $($tail:ty),+) => {
        impl_macro!($t);
        impl_macro!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_macro!($($tail),+);
    };
}

impl_macro!(
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

// ---- special cases we didn't macro-ify  ----
impl<T: TPData> TPData for Vec<T> {}
impl<T: TPData> private::Sealed for Vec<T> {}

impl<T: TPData> TPData for StateHandle<T> {}
impl<T: TPData> private::Sealed for StateHandle<T> {}

impl<T: TPData> TPData for ChannelHandle<T> {}
impl<T: TPData> private::Sealed for ChannelHandle<T> {}
