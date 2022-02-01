macro_rules! __helper {
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
#[doc(hidden)]
pub(crate) use __helper;

macro_rules! DynTpProperty {
    ($ident:ident) => {
        ::paste::paste! {
            $crate::contract::properties::dyn_macro::__helper!([<$ident Single>], $container);
            $crate::contract::properties::dyn_macro::__helper!([<$ident Vec>], $container, Vec);

            #[derive(Debug, Clone, ::derive_more::From)]
            pub enum $ident {
                Single([<$ident Single>]),
                Vec([<$ident Vec>]),
            }
        }
    };
    ($ident:ident, $container:tt) => {
        ::paste::paste! {
            $crate::contract::properties::dyn_macro::__helper!([<$ident Single>], $container);
            $crate::contract::properties::dyn_macro::__helper!([<$ident Vec>], $container, Vec);

            #[derive(Debug, Clone, ::derive_more::From)]
            pub enum $ident {
                Single([<$ident Single>]),
                Vec([<$ident Vec>]),
            }
            impl $ident {
                pub fn new(contract: ContractDataHandle, idx: usize, typ: TpPropertyType) -> Self {
                    use $crate::contract::properties::data::TpDataType;
                    use $crate::object::ObjectHandle;

                    match typ {
                        TpPropertyType::Single(dt) => {
                            let single: [<$ident Single>] = match dt {
                                TpDataType::U8 => $container::<u8>::new(idx, contract).into(),
                                TpDataType::U16 => $container::<u16>::new(idx, contract).into(),
                                TpDataType::U32 => $container::<u32>::new(idx, contract).into(),
                                TpDataType::U64 => $container::<u64>::new(idx, contract).into(),
                                TpDataType::I8 => $container::<i8>::new(idx, contract).into(),
                                TpDataType::I16 => $container::<i16>::new(idx, contract).into(),
                                TpDataType::I32 => $container::<i32>::new(idx, contract).into(),
                                TpDataType::I64 => $container::<i64>::new(idx, contract).into(),
                                TpDataType::Bool => $container::<bool>::new(idx, contract).into(),
                                TpDataType::F32 => $container::<f32>::new(idx, contract).into(),
                                TpDataType::F64 => $container::<f64>::new(idx, contract).into(),
                                TpDataType::String => $container::<String>::new(idx, contract).into(),
                                TpDataType::ObjectHandle => $container::<ObjectHandle>::new(idx, contract).into(),
                                TpDataType::ContractDataHandle => $container::<ContractDataHandle>::new(idx, contract).into(),
                                TpDataType::Dynamic => todo!("how do we handle this?"),
                            };
                            single.into()
                        },
                        TpPropertyType::Vec(dt) => {
                            let vec: [<$ident Vec>] = match dt {
                                TpDataType::U8 => $container::<Vec<u8>>::new(idx, contract).into(),
                                TpDataType::U16 => $container::<Vec<u16>>::new(idx, contract).into(),
                                TpDataType::U32 => $container::<Vec<u32>>::new(idx, contract).into(),
                                TpDataType::U64 => $container::<Vec<u64>>::new(idx, contract).into(),
                                TpDataType::I8 => $container::<Vec<i8>>::new(idx, contract).into(),
                                TpDataType::I16 => $container::<Vec<i16>>::new(idx, contract).into(),
                                TpDataType::I32 => $container::<Vec<i32>>::new(idx, contract).into(),
                                TpDataType::I64 => $container::<Vec<i64>>::new(idx, contract).into(),
                                TpDataType::Bool => $container::<Vec<bool>>::new(idx, contract).into(),
                                TpDataType::F32 => $container::<Vec<f32>>::new(idx, contract).into(),
                                TpDataType::F64 => $container::<Vec<f64>>::new(idx, contract).into(),
                                TpDataType::String => $container::<Vec<String>>::new(idx, contract).into(),
                                TpDataType::ObjectHandle => $container::<Vec<ObjectHandle>>::new(idx, contract).into(),
                                TpDataType::ContractDataHandle => $container::<Vec<ContractDataHandle>>::new(idx, contract).into(),
                                TpDataType::Dynamic => todo!("how do we handle this?"),
                            };
                            vec.into()
                        }
                        TpPropertyType::Dynamic(_) => {
                            todo!("how do we handle this?")
                        }
                    }
                }
            }
        }
    };
}
pub(crate) use DynTpProperty;

// Not part of public API
#[doc(hidden)]
#[macro_export]
macro_rules! __apply_to_prop {
    ($mod_path:path, $prop_type:ident, $dyn_prop:expr, $closure:expr) => {{
        ::paste::paste! {
            use $mod_path::[<$prop_type Single>] as PS;
            use $mod_path::[<$prop_type Vec>] as PV;
        }
        match $dyn_prop {
            $prop_type::Single(s) => match s {
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
                PS::ContractId(id) => $closure(id),
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
                PV::ContractId(id) => $closure(id),
            },
        }
    }};
}
pub(crate) use __apply_to_prop;

/// Applies the provided closure expression to a [`DynStateId`].
///
/// Under the hood, this is matching on the `DynStateId` and the arms in the match
/// expression convert to the corresponding `StateId<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_state {
    ($dyn_state_id:expr, $closure:expr) => {
        $crate::contract::properties::dyn_macro::__apply_to_prop!(
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
        $crate::contract::properties::dyn_macro::__apply_to_prop!(
            $crate::contract::properties::channel,
            DynChannelId,
            $dyn_channel_id,
            $closure
        )
    };
}

pub use apply_to_channel;
pub use apply_to_state;
