#![deny(
    bad_style,
    const_err,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
mod rbox;
mod rvec;
pub mod string;
mod types;

pub use types::{opaque_types, value_types};

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

// This generates the C header file for the bindings. See safer-ffi's guide.
#[safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    let builder = ::safer_ffi::headers::builder();
    if ::std::env::var("HEADERS_TO_STDOUT")
        .ok()
        .map_or(false, |it| it == "1")
    {
        builder.to_writer(::std::io::stdout()).generate()
    } else {
        builder.to_file(&"generated.h".to_string())?.generate()
    }
}
