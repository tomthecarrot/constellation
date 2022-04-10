mod monomorphize;

pub use tp_rsharp_macro::{remangle, substitute};

#[repr(C)]
pub struct MyType {
    a: i32,
    b: f32,
}

impl Drop for MyType {
    fn drop(&mut self) {}
}

#[no_mangle]
pub extern "C" fn MyType__new(a: i32, b: f32) -> *mut MyType {
    let obj = MyType { a: a, b: b };
    Box::into_raw(Box::new(obj))
}

#[no_mangle]
pub extern "C" fn MyType__drop(obj: *mut MyType) {
    drop(obj)
}
