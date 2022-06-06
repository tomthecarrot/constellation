#[macro_export]
macro_rules! rvec_fns {
    ($path:literal, $t:ty) => {
        paste::paste! {
            #[::rsharp::remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __push>](vec: &mut ::safer_ffi::vec::Vec<$t>, item: ::safer_ffi::boxed::Box<$t>) {
                let item: $t = *item.into();
                vec.with_rust_mut(|vec| vec.push(item))
            }

            #[::rsharp::remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __new>]() -> ::safer_ffi::vec::Vec<$t> {
                ::safer_ffi::vec::Vec::EMPTY
            }

            #[::rsharp::remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __drop>](vec: ::safer_ffi::vec::Vec<$t>) {
                drop(vec)
            }

            #[::rsharp::remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __get>]<'a>(vec: &'a ::safer_ffi::vec::Vec<$t>, idx: usize) -> &'a $t {
                &vec[idx]
            }

            #[::rsharp::remangle($path)]
            #[::safer_ffi::ffi_export]
            pub fn [<RVec_ $t:camel __set>]<'a>(vec: &'a mut ::safer_ffi::vec::Vec<$t>, idx: usize, value: ::safer_ffi::boxed::Box<$t>) {
                vec[idx] = *value.into();
            }
        }
    };
}
