/// Creates enum variants for each primitive type
macro_rules! dyn_enum_helper {
    ($ident:ident$( | $attr:meta)?) => {
        #[derive(Debug, ::derive_more::From, ::derive_more::TryInto)]
        $(#[$attr])?
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
    ($ident:ident, $t:tt$( | $attr:meta)?) => {
        #[derive(Debug, ::derive_more::From, ::derive_more::TryInto)]
        $(#[$attr])?
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
    ($ident:ident, $outer:tt, $inner:tt$( | $attr:meta)?) => {
        #[derive(Debug, ::derive_more::From, ::derive_more::TryInto)]
        $(#[$attr])?
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

// pub only due to its use in `DynEnum`
#[doc(hidden)]
pub(in crate::contract::properties) use dyn_enum_helper;

/// Creates an enum that holds all possible types, possibly wrapped by a `container` type
macro_rules! DynEnum {
    ($ident:ident, $container:tt, $prim_ident:ident, $vec_ident:ident$( | $attr:meta)?) => {
        $crate::contract::properties::dynamic::__macro::dyn_enum_helper!($prim_ident, $container$( | $attr)?);
        $crate::contract::properties::dynamic::__macro::dyn_enum_helper!(
            $vec_ident, $container, Vec$( | $attr)?
        );

        #[derive(Debug, ::derive_more::From, ::derive_more::TryInto)]
        $(#[$attr])?
        pub enum $ident {
            Primitive($prim_ident),
            Vec($vec_ident),
        }
    };
    ($ident:ident, $prim_ident:ident, $vec_ident:ident$( | $attr:meta)?) => {
        $crate::contract::properties::dynamic::__macro::dyn_enum_helper!($prim_ident$( | $attr)?);
        $crate::contract::properties::dynamic::__macro::dyn_enum_helper!($vec_ident, Vec$( | $attr)?);

        #[derive(Debug, ::derive_more::From, ::derive_more::TryInto)]
        $(#[$attr])?
        pub enum $ident {
            Primitive($prim_ident),
            Vec($vec_ident),
        }
    };
    ($ident:ident, $container:tt$( | $attr:meta)?) => {
        ::paste::paste! {
            DynEnum!($ident, $container, [<$ident Primitive>], [<$ident Vec>]$( | $attr)?);
        }
    };
    ($ident:ident$( | $attr:meta)?) => {
        ::paste::paste! {
            DynEnum!($ident, [<$ident Primitive>], [<$ident Vec>]$( | $attr)?);
        }
    };
}
#[doc(hidden)]
pub(in crate::contract::properties) use DynEnum;

/// Calls `DynEnum` but adds PropertyId specific functions
macro_rules! DynTpPropId {
    ($ident:ident, $container:tt) => {
        DynEnum!($ident, $container);

        ::paste::paste! {
            impl $ident {
                pub fn new(contract: $crate::contract::ContractDataHandle, idx: usize, typ: $crate::contract::properties::dynamic::TpPropertyType) -> Self {
                    use $crate::contract::properties::dynamic::TpPrimitiveType;
                    use $crate::object::ObjectHandle;
                    use $crate::contract::properties::dynamic::TpPropertyType;

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

/// Applies the provided `closure` to an enum generated by `DynEnum`. Used by other
/// macros that simplify the arguments. Not intended for use in the public API, and
/// is only exported due to necessity
#[doc(hidden)]
#[macro_export]
macro_rules! apply_to_dyn {
    ($mod_path:path, $enum_type:ident, $prim_type:ident, $vec_type:ident, $dyn_prop:expr, $closure:expr) => {{
        ::paste::paste! {
            use $mod_path::$enum_type;
            use $mod_path::$prim_type as PS;
            use $mod_path::$vec_type as PV;
        }
        match $dyn_prop {
            $enum_type::Primitive(s) => match s {
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
            $enum_type::Vec(s) => match s {
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
pub(crate) use apply_to_dyn;

/// Applies the provided closure expression to a [`DynStateId`].
///
/// Under the hood, this is matching on the `DynStateId` and the arms in the match
/// expression convert to the corresponding `StateId<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_state_id {
    ($dyn_state_id:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::states::id,
            DynStateId,
            DynStateIdPrimitive,
            DynStateIdVec,
            $dyn_state_id,
            $closure
        )
    };
}
pub use apply_to_state_id;

/// Applies the provided closure expression to a [`DynChannelId`].
///
/// Under the hood, this is matching on the `DynChannelId` and the arms in the match
/// expression convert to the corresponding `ChannelId<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_channel_id {
    ($dyn_channel_id:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::channels,
            DynChannelId,
            DynChannelIdPrimitive,
            DynChannelIdVec,
            $dyn_channel_id,
            $closure
        )
    };
}
pub use apply_to_channel_id;

/// Applies the provided closure expression to a [`DynTpProperty`].
///
/// Under the hood, this is matching on the `DynTpProperty` and the arms in the match
/// expression convert to the corresponding concrete type.
#[macro_export]
macro_rules! apply_to_prop {
    ($dyn_prop:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::dynamic,
            DynTpProperty,
            DynTpPrimitive,
            DynTpVec,
            $dyn_prop,
            $closure
        )
    };
}
pub use apply_to_prop;

/// Applies the provided closure expression to a [`DynChannel`].
///
/// Under the hood, this is matching on the `DynChannel` and the arms in the match
/// expression convert to the corresponding concrete type.
#[macro_export]
macro_rules! apply_to_channel {
    ($dyn_prop:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::channels::dyn_channel,
            DynChannel,
            DynChannelPrimitive,
            DynChannelVec,
            $dyn_prop,
            $closure
        )
    };
}
pub use apply_to_channel;

/// Applies the provided closure expression to a [`DynChannelRef`].
///
/// Under the hood, this is matching on the `DynChannelRef` and the arms in the match
/// expression convert to the corresponding concrete type.
#[macro_export]
macro_rules! apply_to_channel_ref {
    ($dyn_prop:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::channels::dyn_channel,
            DynChannelRef,
            DynChannelPrimitiveRef,
            DynChannelVecRef,
            $dyn_prop,
            $closure
        )
    };
}
pub use apply_to_channel_ref;

/// Applies the provided closure expression to a [`DynChannelMut`].
///
/// Under the hood, this is matching on the `DynChannelMut` and the arms in the match
/// expression convert to the corresponding concrete type.
#[macro_export]
macro_rules! apply_to_channel_mut {
    ($dyn_prop:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::channels::dyn_channel,
            DynChannelMut,
            DynChannelPrimitiveMut,
            DynChannelVecMut,
            $dyn_prop,
            $closure
        )
    };
}
pub use apply_to_channel_mut;

/// Applies the provided closure expression to a [`DynStateHandle`].
///
/// Under the hood, this is matching on the `DynStateHandle` and the arms in the match
/// expression convert to the corresponding `StateHandle<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_state_handle {
    ($dyn_handle:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::states::dyn_handle,
            DynStateHandle,
            DynStateHandlePrimitive,
            DynStateHandleVec,
            $dyn_handle,
            $closure
        )
    };
}
pub use apply_to_state_handle;

/// Applies the provided closure expression to a [`DynChannelHandle`].
///
/// Under the hood, this is matching on the `DynChannelHandle` and the arms in the match
/// expression convert to the corresponding `ChannelHandle<T>`, filling in the `T`.
#[macro_export]
macro_rules! apply_to_channel_handle {
    ($dyn_handle:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::channels::dyn_handle,
            DynChannelHandle,
            DynChannelHandlePrimitive,
            DynChannelHandleVec,
            $dyn_handle,
            $closure
        )
    };
}
pub use apply_to_channel_handle;

/// Applies the provided closure expression to a [`DynTpPropertyRef`].
///
/// Under the hood, this is matching on the `DynTpPropertyRef` and the arms in the match
/// expression convert to the corresponding concrete type.
#[macro_export]
macro_rules! apply_to_prop_ref {
    ($dyn_prop_ref:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::dynamic::property,
            DynTpPropertyRef,
            DynTpPrimitiveRef,
            DynTpVecRef,
            $dyn_prop_ref,
            $closure
        )
    };
}
pub use apply_to_prop_ref;

/// Applies the provided closure expression to a [`DynTpPropertyMut`].
///
/// Under the hood, this is matching on the `DynTpPropertyMut` and the arms in the match
/// expression convert to the corresponding concrete type.
#[macro_export]
macro_rules! apply_to_prop_mut {
    ($dyn_prop_mut:expr, $closure:expr) => {
        $crate::contract::properties::dynamic::__macro::apply_to_dyn!(
            $crate::contract::properties::dynamic::property,
            DynTpPropertyMut,
            DynTpPrimitiveMut,
            DynTpVecMut,
            $dyn_prop_mut,
            $closure
        )
    };
}
pub use apply_to_prop_mut;
