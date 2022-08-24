/// Types that can be used in FFI when behind a pointer, also known as "opaque" types.
pub mod opaque_types {
    pub use crate::string::String;

    macro_rules! opaque_types {
        (; types, $macro_name:ident, $($x:tt)+) => {
            $macro_name!(
                $($x)+,
                String,
            );
        };
        (; types, $macro_name:ident) => {
            $macro_name!(
                String,
            );
        };
    }

    pub(crate) use opaque_types;
}

/// Types that can be used in FFI without a pointer. Also known as "value" types.
pub mod value_types {
    pub use bool;
    pub use f32;
    pub use f64;
    pub use i16;
    pub use i32;
    pub use i64;
    pub use i8;
    pub use u16;
    pub use u32;
    pub use u64;
    pub use u8;

    macro_rules! value_types {
        (; types, $macro_name:ident, $($x:tt)+) => {
            $macro_name!(
                $($x)+,
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
            );
        };
        (; types, $macro_name:ident) => {
            $macro_name!(
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
            );
        };
    }
    pub(crate) use value_types;
}
