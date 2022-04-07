#[macro_export]
macro_rules! monomorphize {
    // Base case
    ($path:literal, $item_name:ident, $t:ty $(,)?) => {
        paste::paste! {
            #[allow(non_camel_case_types, dead_code)]
            #[$crate::remangle($path)]
            #[repr(C)]
            pub struct [<$item_name _ $t:camel>](pub $item_name<$t>);

            // #[$crate::remangle($path)]
            // #[no_mangle]
            // pub extern "C" fn get_keyframe_value_ <$t> (keyframe_c: [<$item_name _ $t:camel>]) -> $t {
            //     keyframe_c.0.value
            // }
        }
    };
    // recursive case
    ($path:literal, $item_name:ident, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
        monomorphize!($path, $item_name, $first_t);
        monomorphize!($path, $item_name, $($tail_t),+);
    };
}

#[cfg(test)]
pub mod tests {

    #[derive(PartialEq, Eq, Debug)]
    pub struct MyGeneric<T>(T);

    #[test]
    pub fn test1() {
        monomorphize!("my::path", MyGeneric, u8, f32, String);

        assert_eq!(my__path__MyGeneric_U8(MyGeneric(10u8)).0, MyGeneric(10u8));
        assert_eq!(
            my__path__MyGeneric_F32(MyGeneric(10f32)).0,
            MyGeneric(10f32)
        );
        assert_eq!(
            my__path__MyGeneric_String(MyGeneric("hello".to_string())).0,
            MyGeneric("hello".to_string())
        );
    }
}
