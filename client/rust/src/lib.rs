#![deny(
    bad_style,
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
#![allow(clippy::diverging_sub_expression)]

// Necessary to allow proc macros to have the correct crate name when invoked from
// this crate
extern crate self as tp_client;

/// Reexported for the sake of the proc macros
mod re_exports {
    pub use ::lazy_static;
    pub use ::paste;
}
pub use self::re_exports::*;

pub mod action;
pub mod baseline;
pub mod contract;
pub mod engine;
pub mod object;
pub mod realm;
pub mod time;

pub use engine::Engine;

// This generates the C header file for the bindings. See safer-ffi's guide.
#[cfg(feature = "c_api")]
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
