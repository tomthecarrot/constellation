macro_rules! helper {
    ($ident:ident) => {
        #[derive(Debug, Clone, ::derive_more::From)]
        pub enum $ident {
            U8(u8),
            U16(u16),
            U32(u32),
            U64(u64),
            I8(i8),
            I16(i16),
            I32(i32),
            I64(i64),
            Bool(bool),
            F32(f32),
            F64(f64),
            String(String),
            ObjectHandle($crate::object::ObjectHandle),
            ContractId($crate::contract::ContractDataHandle),
        }
    };
    ($ident:ident, $t:tt) => {
        #[derive(Debug, Clone, ::derive_more::From)]
        pub enum $ident {
            U8($t<u8>),
            U16($t<u16>),
            U32($t<u32>),
            U64($t<u64>),
            I8($t<i8>),
            I16($t<i16>),
            I32($t<i32>),
            I64($t<i64>),
            Bool($t<bool>),
            F32($t<f32>),
            F64($t<f64>),
            String($t<String>),
            ObjectHandle($t<$crate::object::ObjectHandle>),
            ContractId($t<$crate::contract::ContractDataHandle>),
        }
    };
    ($ident:ident, $outer:tt, $inner:tt) => {
        #[derive(Debug, Clone, ::derive_more::From)]
        pub enum $ident {
            U8($outer<$inner<u8>>),
            U16($outer<$inner<u16>>),
            U32($outer<$inner<u32>>),
            U64($outer<$inner<u64>>),
            I8($outer<$inner<i8>>),
            I16($outer<$inner<i16>>),
            I32($outer<$inner<i32>>),
            I64($outer<$inner<i64>>),
            Bool($outer<$inner<bool>>),
            F32($outer<$inner<f32>>),
            F64($outer<$inner<f64>>),
            String($outer<$inner<String>>),
            ObjectHandle($outer<$inner<$crate::object::ObjectHandle>>),
            ContractId($outer<$inner<$crate::contract::ContractDataHandle>>),
        }
    };
}

macro_rules! DynTpProperty {
    ($ident:ident) => {
        ::paste::paste! {
            $crate::contract::properties::dyn_macro::helper!([<$ident Single>], $container);
            $crate::contract::properties::dyn_macro::helper!([<$ident Vec>], $container, Vec);

            #[derive(Debug, Clone, ::derive_more::From)]
            pub enum $ident {
                Single([<$ident Single>]),
                Vec([<$ident Vec>]),
            }
        }
    };
    ($ident:ident, $container:ty) => {
        ::paste::paste! {
            $crate::contract::properties::dyn_macro::helper!([<$ident Single>], $container);
            $crate::contract::properties::dyn_macro::helper!([<$ident Vec>], $container, Vec);

            #[derive(Debug, Clone, ::derive_more::From)]
            pub enum $ident {
                Single([<$ident Single>]),
                Vec([<$ident Vec>]),
            }
        }
    };
}

pub(crate) use helper;
pub(crate) use DynTpProperty;
