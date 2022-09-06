// TODO[SER-338] uncomment me // #![deny(improper_ctypes_definitions, improper_ctypes)]
#![allow(clippy::diverging_sub_expression)]
#![feature(generic_associated_types)]
#![feature(trivial_bounds)]

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
