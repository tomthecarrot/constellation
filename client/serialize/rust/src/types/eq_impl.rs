use crate::rs;

impl PartialEq<rs::TpPropertyType> for crate::primitive::TpPrimitiveKind {
    fn eq(&self, other: &rs::TpPropertyType) -> bool {
        use crate::primitive::TpPrimitiveKind as T;
        use rs::TpPrimitiveType as C;
        use rs::TpPropertyType::Primitive;
        match (*self, other) {
            (T::U8, Primitive(C::U8))
            | (T::U16, Primitive(C::U16))
            | (T::U32, Primitive(C::U32))
            | (T::U64, Primitive(C::U64))
            | (T::I8, Primitive(C::I8))
            | (T::I16, Primitive(C::I16))
            | (T::I32, Primitive(C::I32))
            | (T::I64, Primitive(C::I64))
            | (T::Bool, Primitive(C::Bool))
            | (T::F32, Primitive(C::F32))
            | (T::F64, Primitive(C::F64))
            | (T::String, Primitive(C::String))
            | (T::ObjectHandle, Primitive(C::ObjectHandle))
            | (T::ContractDataHandle, Primitive(C::ContractDataHandle)) => true,
            _ => false,
        }
    }
}
impl PartialEq<crate::primitive::TpPrimitiveKind> for rs::TpPropertyType {
    fn eq(&self, other: &crate::primitive::TpPrimitiveKind) -> bool {
        other == self
    }
}

impl PartialEq<rs::TpPropertyType> for crate::primitive::TpPrimitive {
    fn eq(&self, other: &rs::TpPropertyType) -> bool {
        use rs::TpPrimitiveType as C;
        use rs::TpPropertyType::Primitive;
        match (*self, *other) {
            (Self::U8, Primitive(C::U8))
            | (Self::U16, Primitive(C::U16))
            | (Self::U32, Primitive(C::U32))
            | (Self::U64, Primitive(C::U64))
            | (Self::I8, Primitive(C::I8))
            | (Self::I16, Primitive(C::I16))
            | (Self::I32, Primitive(C::I32))
            | (Self::I64, Primitive(C::I64))
            | (Self::Bool, Primitive(C::Bool))
            | (Self::F32, Primitive(C::F32))
            | (Self::F64, Primitive(C::F64))
            | (Self::FbString, Primitive(C::String))
            | (Self::tp_serialize_object_ObjectHandle, Primitive(C::ObjectHandle))
            | (Self::tp_serialize_contract_ContractDataHandle, Primitive(C::ContractDataHandle)) => {
                true
            }
            _ => false,
        }
    }
}
impl PartialEq<crate::primitive::TpPrimitive> for rs::TpPropertyType {
    fn eq(&self, other: &crate::primitive::TpPrimitive) -> bool {
        other == self
    }
}

impl PartialEq<crate::primitive::TpPrimitiveKind> for crate::primitive::TpPrimitive {
    fn eq(&self, other: &crate::primitive::TpPrimitiveKind) -> bool {
        use crate::primitive::TpPrimitiveKind as O;
        match (*self, *other) {
            (Self::U8, O::U8)
            | (Self::U16, O::U16)
            | (Self::U32, O::U64)
            | (Self::I8, O::I8)
            | (Self::I16, O::I16)
            | (Self::I32, O::I32)
            | (Self::I64, O::I64)
            | (Self::Bool, O::Bool)
            | (Self::F32, O::F32)
            | (Self::F64, O::F64)
            | (Self::FbString, O::String)
            | (Self::tp_serialize_object_ObjectHandle, O::ObjectHandle)
            | (Self::tp_serialize_contract_ContractDataHandle, O::ContractDataHandle) => true,
            _ => false,
        }
    }
}
impl PartialEq<crate::primitive::TpPrimitive> for crate::primitive::TpPrimitiveKind {
    fn eq(&self, other: &crate::primitive::TpPrimitive) -> bool {
        other == self
    }
}
