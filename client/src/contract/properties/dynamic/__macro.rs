macro_rules! DynEnum {
    ($ident:ident) => {
        #[derive(Debug, Clone, PartialEq, ::derive_more::From, ::derive_more::TryInto)]
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
            ContractDataHandle($crate::contract::ContractDataHandle),
        }
    };
    ($ident:ident, $t:tt) => {
        #[derive(Debug, Clone, PartialEq, ::derive_more::From, ::derive_more::TryInto)]
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
            ContractDataHandle($t<$crate::contract::ContractDataHandle>),
        }
    };
    ($ident:ident, $outer:tt, $inner:tt) => {
        #[derive(Debug, Clone, PartialEq, ::derive_more::From, ::derive_more::TryInto)]
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
            ContractDataHandle($outer<$inner<$crate::contract::ContractDataHandle>>),
        }
    };
}
#[doc(hidden)]
pub(in crate::contract::properties) use DynEnum;

macro_rules! DynTpPropId {
    ($ident:ident, $container:tt) => {
        ::paste::paste! {
            $crate::contract::properties::dynamic::__macro::DynEnum!([<$ident Primitive>], $container);
            $crate::contract::properties::dynamic::__macro::DynEnum!([<$ident Vec>], $container, Vec);

            #[derive(Debug, Clone, PartialEq, ::derive_more::From, ::derive_more::TryInto)]
            pub enum $ident {
                Primitive([<$ident Primitive>]),
                Vec([<$ident Vec>]),
            }
            impl $ident {
                pub fn new(contract: ContractDataHandle, idx: usize, typ: TpPropertyType) -> Self {
                    use $crate::contract::properties::dynamic::TpPrimitiveType;
                    use $crate::object::ObjectHandle;

                    match typ {
                        TpPropertyType::Primitive(dt) => {
                            let single: [<$ident Primitive>] = match dt {
                                TpPrimitiveType::U8 => $container::<u8>::new(idx, contract).into(),
                                TpPrimitiveType::U16 => $container::<u16>::new(idx, contract).into(),
                                TpPrimitiveType::U32 => $container::<u32>::new(idx, contract).into(),
                                TpPrimitiveType::U64 => $container::<u64>::new(idx, contract).into(),
                                TpPrimitiveType::I8 => $container::<i8>::new(idx, contract).into(),
                                TpPrimitiveType::I16 => $container::<i16>::new(idx, contract).into(),
                                TpPrimitiveType::I32 => $container::<i32>::new(idx, contract).into(),
                                TpPrimitiveType::I64 => $container::<i64>::new(idx, contract).into(),
                                TpPrimitiveType::Bool => $container::<bool>::new(idx, contract).into(),
                                TpPrimitiveType::F32 => $container::<f32>::new(idx, contract).into(),
                                TpPrimitiveType::F64 => $container::<f64>::new(idx, contract).into(),
                                TpPrimitiveType::String => $container::<String>::new(idx, contract).into(),
                                TpPrimitiveType::ObjectHandle => $container::<ObjectHandle>::new(idx, contract).into(),
                                TpPrimitiveType::ContractDataHandle => $container::<ContractDataHandle>::new(idx, contract).into(),
                            };
                            single.into()
                        },
                        TpPropertyType::Vec(dt) => {
                            let vec: [<$ident Vec>] = match dt {
                                TpPrimitiveType::U8 => $container::<Vec<u8>>::new(idx, contract).into(),
                                TpPrimitiveType::U16 => $container::<Vec<u16>>::new(idx, contract).into(),
                                TpPrimitiveType::U32 => $container::<Vec<u32>>::new(idx, contract).into(),
                                TpPrimitiveType::U64 => $container::<Vec<u64>>::new(idx, contract).into(),
                                TpPrimitiveType::I8 => $container::<Vec<i8>>::new(idx, contract).into(),
                                TpPrimitiveType::I16 => $container::<Vec<i16>>::new(idx, contract).into(),
                                TpPrimitiveType::I32 => $container::<Vec<i32>>::new(idx, contract).into(),
                                TpPrimitiveType::I64 => $container::<Vec<i64>>::new(idx, contract).into(),
                                TpPrimitiveType::Bool => $container::<Vec<bool>>::new(idx, contract).into(),
                                TpPrimitiveType::F32 => $container::<Vec<f32>>::new(idx, contract).into(),
                                TpPrimitiveType::F64 => $container::<Vec<f64>>::new(idx, contract).into(),
                                TpPrimitiveType::String => $container::<Vec<String>>::new(idx, contract).into(),
                                TpPrimitiveType::ObjectHandle => $container::<Vec<ObjectHandle>>::new(idx, contract).into(),
                                TpPrimitiveType::ContractDataHandle => $container::<Vec<ContractDataHandle>>::new(idx, contract).into(),
                            };
                            vec.into()
                        }
                    }
                }
            }
        }
    };
}
pub(in crate::contract::properties) use DynTpPropId;

// Not part of public API
#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_prop {
    ($mod_path:path, $prop_type:ident, $dyn_prop:expr, $closure:expr) => {{
        ::paste::paste! {
            use $mod_path::[<$prop_type Primitive>] as PS;
            use $mod_path::[<$prop_type Vec>] as PV;
        }
        match $dyn_prop {
            $prop_type::Primitive(s) => match s {
                PS::U8(id) => $closure(id),
                PS::U16(id) => $closure(id),
                PS::U32(id) => $closure(id),
                PS::U64(id) => $closure(id),
                PS::I8(id) => $closure(id),
                PS::I16(id) => $closure(id),
                PS::I32(id) => $closure(id),
                PS::I64(id) => $closure(id),
                PS::Bool(id) => $closure(id),
                PS::F32(id) => $closure(id),
                PS::F64(id) => $closure(id),
                PS::String(id) => $closure(id),
                PS::ObjectHandle(id) => $closure(id),
                PS::ContractDataHandle(id) => $closure(id),
            },
            $prop_type::Vec(s) => match s {
                PV::U8(id) => $closure(id),
                PV::U16(id) => $closure(id),
                PV::U32(id) => $closure(id),
                PV::U64(id) => $closure(id),
                PV::I8(id) => $closure(id),
                PV::I16(id) => $closure(id),
                PV::I32(id) => $closure(id),
                PV::I64(id) => $closure(id),
                PV::Bool(id) => $closure(id),
                PV::F32(id) => $closure(id),
                PV::F64(id) => $closure(id),
                PV::String(id) => $closure(id),
                PV::ObjectHandle(id) => $closure(id),
                PV::ContractDataHandle(id) => $closure(id),
            },
        }
    }};
}
pub(crate) use apply_to_prop;

/// Applies the provided closure expression to a [`DynStateId`].
///
/// Under the hood, this is matching on the `DynStateId` and the arms in the match
/// expression convert to the corresponding `StateId<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_state {
    ($dyn_state_id:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_prop!(
            $crate::contract::properties::state,
            DynStateId,
            $dyn_state_id,
            $closure
        )
    };
}

/// Applies the provided closure expression to a [`DynChannelId`].
///
/// Under the hood, this is matching on the `DynChannelId` and the arms in the match
/// expression convert to the corresponding `ChannelId<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_channel {
    ($dyn_channel_id:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_prop!(
            $crate::contract::properties::channel,
            DynChannelId,
            $dyn_channel_id,
            $closure
        )
    };
}

pub use apply_to_channel;
pub use apply_to_state;
