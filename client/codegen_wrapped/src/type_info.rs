use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ValidOwnershipSemantics {
    Owned,
    Ref,
    Both,
}

#[enum_dispatch]
pub trait TypeInfo: Copy + Clone + Debug + Eq + PartialEq + Hash {
    /// The `F32` or `Bool
    fn mangled_name(&self) -> &'static str;

    /// The `IntPtr` or `float*`
    fn ptr_raw(&self) -> &'static str;

    /// The `ObjectHandle` or `RBox_F32`
    fn owned_ident(&self) -> &'static str;

    /// If the type can be constructed from C# directly
    fn supports_new(&self) -> bool;

    /// The `T` in Ptr<T>
    fn ptr_inner(&self) -> &'static str;

    fn valid_ownership_semantics(&self) -> ValidOwnershipSemantics;
}
pub trait TypeInfoConcrete: TypeInfo + Into<PrimitiveType> + Default {
    fn new() -> Self {
        Self::default()
    }
}

#[enum_dispatch(TypeInfo)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PrimitiveType {
    U8(U8),
    U16(U16),
    U32(U32),
    U64(U64),
    I8(I8),
    I16(I16),
    I32(I32),
    I64(I64),
    Bool(Bool),
    F32(F32),
    F64(F64),
    ObjectHandle(ObjectHandle),
    ContractDataHandle(ContractDataHandle),
}
impl PrimitiveType {
    pub fn new<T: TypeInfoConcrete>() -> Self {
        T::new().into()
    }
    pub fn types() -> &'static [PrimitiveType] {
        use PrimitiveType as P;
        lazy_static! {
            static ref RESULT: [PrimitiveType; 13] = [
                P::new::<U8>(),
                P::new::<U16>(),
                P::new::<U32>(),
                P::new::<U64>(),
                P::new::<I8>(),
                P::new::<I16>(),
                P::new::<I32>(),
                P::new::<I64>(),
                P::new::<Bool>(),
                P::new::<F32>(),
                P::new::<F64>(),
                P::new::<ObjectHandle>(),
                P::new::<ContractDataHandle>(),
            ];
        }
        &*RESULT
    }
}

macro_rules! type_info {
    ($t:ident, $mangled_name:literal, $ptr_raw:literal, $owned_ident:literal, $supports_new:literal, $ptr_inner:literal, $valid_o:expr$(,)?) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
        pub struct $t;
        impl_t!(
            $t,
            $mangled_name,
            $ptr_raw,
            $owned_ident,
            $supports_new,
            $ptr_inner,
            $valid_o,
        );
    };
}
macro_rules! impl_t {
    ($t:ty, $mangled_name:literal, $ptr_raw:literal, $owned_ident:literal, $supports_new:literal, $ptr_inner:literal, $valid_o:expr$(,)?) => {
        impl TypeInfo for $t {
            fn mangled_name(&self) -> &'static str {
                $mangled_name
            }

            fn ptr_raw(&self) -> &'static str {
                $ptr_raw
            }

            fn owned_ident(&self) -> &'static str {
                $owned_ident
            }

            fn supports_new(&self) -> bool {
                $supports_new
            }

            fn ptr_inner(&self) -> &'static str {
                $ptr_inner
            }

            fn valid_ownership_semantics(&self) -> ValidOwnershipSemantics {
                $valid_o
            }
        }
        impl TypeInfoConcrete for $t {}
    };
}

use ValidOwnershipSemantics as O;

type_info!(U8, "U8", "byte*", "RBox_U8", true, "byte", O::Both);
type_info!(U16, "U16", "ushort*", "RBox_U16", true, "ushort", O::Both);
type_info!(U32, "U32", "uint*", "RBox_U32", true, "uint", O::Both);
type_info!(U64, "U64", "ulong*", "RBox_U64", true, "ulong", O::Both);
type_info!(I8, "I8", "sbyte*", "RBox_I8", true, "sbyte", O::Both);
type_info!(I16, "I16", "short*", "RBox_I16", true, "short", O::Both);
type_info!(I32, "I32", "int*", "RBox_I32", true, "int", O::Both);
type_info!(I64, "I64", "long*", "RBox_I64", true, "long", O::Both);
type_info!(Bool, "Bool", "bool*", "RBox_Bool", true, "bool", O::Both);
type_info!(F32, "F32", "float*", "RBox_F32", true, "float", O::Both);
type_info!(F64, "F64", "double*", "RBox_F64", true, "double", O::Both);
type_info!(
    ObjectHandle,
    "ObjectHandle",
    "IntPtr",
    "Teleportal.Client.Object.ObjectHandle",
    false,
    "Teleportal.Client.Object.ObjectHandle",
    O::Owned,
);
type_info!(
    ContractDataHandle,
    "ContractDataHandle",
    "IntPtr",
    "Teleportal.Client.Contract.ContractDataHandle",
    false,
    "Teleportal.Client.Contract.ContractDataHandle",
    O::Owned,
);
