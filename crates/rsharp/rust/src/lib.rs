mod primitives;
mod string;
mod vec;

pub use rsharp_macro::{remangle, substitute};

/// Provides additional helper functions on `Option<T>`.
pub trait OptionExt<T>: private::Sealed
where
    Self: Sized,
{
    /// Equivalent to `self.expect("unexpected null pointer!")`.
    /// Useful for conversion from Option<&T> to &T in ffi signatures.
    fn expect_not_null(self) -> T;
}
impl<'a, T> OptionExt<&'a T> for Option<&'a T> {
    fn expect_not_null(self) -> &'a T {
        self.expect("unexpected null pointer!")
    }
}
impl<'a, T> OptionExt<&'a mut T> for Option<&'a mut T> {
    fn expect_not_null(self) -> &'a mut T {
        self.expect("unexpected null pointer!")
    }
}
mod private {
    pub trait Sealed {}
    impl<T> Sealed for Option<T> {}
}
