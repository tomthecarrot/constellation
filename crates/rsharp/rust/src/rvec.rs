#[macro_export]
macro_rules! rvec_fns {
    ($path:literal, $t:ty) => {
        paste::paste! {
            #[remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __push>](v: &mut ::safer_ffi::vec::Vec<$t>, item: ::safer_ffi::boxed::Box<$t>) {
                let item: $t = *item.into();
                v.with_rust_mut(|v| v.push(item))
            }

            #[remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __new>]() -> ::safer_ffi::vec::Vec<$t> {
                ::safer_ffi::vec::Vec::EMPTY
            }

            #[remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __drop>](v: ::safer_ffi::vec::Vec<$t>) {
                drop(v)
            }
        }
    };
}
