/// Any data that can be stored in a property implements `TPData`.
pub trait TPData: Sized + 'static + private::Sealed {}

impl TPData for u8 {}

mod private {
    /// Prevents trait implementation by third parties. See
    /// https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait Sealed {}

    impl Sealed for u8 {}
}
