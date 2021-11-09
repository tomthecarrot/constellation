mod channel;
mod data;
mod state;

pub use channel::{Channel, ChannelArenaHandle, ChannelArenaMap, ChannelHandle, ChannelID};
pub use data::TPData;
pub use state::{State, StateArenaHandle, StateArenaMap, StateHandle, StateID};

/*

/// Any type that can be stored in an object property must implement `Property`.
///
/// *NOTE*: The `'de` lifetime enables deserializing a `Property` to be a zero
/// copy operation, keeping a reference to the original binary data that it
/// doesn't own. For more info, read [here](https://serde.rs/lifetimes.html)
///
/// This trait is sealed and cannot be implemented by third party crates.
pub trait Property<'de>: serde::Deserialize<'de> + serde::Serialize + private::Sealed {}

// ---- Property trait impls ----

impl<'d> Property<'d> for u8 {}
impl<'d> Property<'d> for u16 {}
impl<'d> Property<'d> for u32 {}
impl<'d> Property<'d> for u64 {}

impl<'d> Property<'d> for i8 {}
impl<'d> Property<'d> for i16 {}
impl<'d> Property<'d> for i32 {}
impl<'d> Property<'d> for i64 {}

impl<'d> Property<'d> for f32 {}
impl<'d> Property<'d> for f64 {}

impl<'d> Property<'d> for bool {}

impl<'d> Property<'d> for String {}
impl<'d: 'a, 'a> Property<'d> for &'a str {}

impl<'d, T> Property<'d> for Vec<T> where T: Property<'d> {}
impl<'d: 'a, 'a> Property<'d> for &'a [u8] {}

impl<'d, T> Property<'d> for Option<T> where T: Property<'d> {}

impl<'d> Property<'d> for std::time::Duration {}

impl<'d> Property<'d> for crate::object::ObjectID {}

mod private {
    /// Sealed traits cannot be implemented by modules that do not have
    /// visibility on `Sealed`
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}

    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}

    impl Sealed for f32 {}
    impl Sealed for f64 {}

    impl Sealed for bool {}

    impl Sealed for String {}
    impl Sealed for &str {}

    impl<T> Sealed for Vec<T> where T: Sealed {}
    impl Sealed for  {}
    impl Sealed for u8 {}
    impl Sealed for u8 {}
    impl Sealed for u8 {}
}

*/
