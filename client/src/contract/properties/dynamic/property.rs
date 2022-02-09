use super::primitive::{DynTpPrimitive, TpPrimitiveType};
use crate::contract::properties::traits::ITpProperty;

use super::vec::DynTpVec;

use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

use derive_more::{From, TryInto};

/// The static type of the ITpPropertyStatic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpPropertyType {
    Vec(TpPrimitiveType),
    Primitive(TpPrimitiveType),
}
impl TpPropertyType {
    const fn primitive_type(&self) -> TpPrimitiveType {
        match self {
            Self::Vec(pt) => *pt,
            Self::Primitive(pt) => *pt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, From, TryInto)]
pub enum DynTpProperty {
    Primitive(DynTpPrimitive),
    Vec(DynTpVec),
}

impl DynTpProperty {
    pub const fn prop_type(&self) -> TpPropertyType {
        match self {
            Self::Primitive(tpp) => tpp.prop_type(),
            Self::Vec(tpv) => tpv.prop_type(),
        }
    }
}

impl ITpProperty for DynTpProperty {
    fn prop_type(&self) -> TpPropertyType {
        self.prop_type()
    }
}

// ---- PartialEq and PartialOrd impls ----

macro_rules! impl_equality {
    // base case
    ($t:ty) => {
        // ---- Single variants ----
        impl PartialEq<$t> for DynTpProperty {
            fn eq(&self, other: &$t) -> bool {
                if let Self::Primitive(inner) = self {
                    inner == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<DynTpProperty> for $t {
            fn eq(&self, other: &DynTpProperty) -> bool {
                if let DynTpProperty::Primitive(other) = other {
                    self == other
                } else {
                    false
                }
            }
        }

        // ---- Vec variants ----

        impl PartialEq<Vec<$t>> for DynTpProperty {
            fn eq(&self, other: &Vec<$t>) -> bool {
                if let Self::Vec(inner) = self {
                    inner == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<DynTpProperty> for Vec<$t> {
            fn eq(&self, other: &DynTpProperty) -> bool {
                if let DynTpProperty::Vec(other) = other {
                    self == other
                } else {
                    false
                }
            }
        }

    };
    // recursive case
    ($t:ty, $($tail:ty),+) => {
        impl_equality!($t);
        impl_equality!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_equality!($($tail),+);
    };
}

impl_equality!(
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
    String,
    ObjectHandle,
    ContractDataHandle,
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partialeq() {
        let u16_1 = DynTpProperty::Primitive(1u16.into());
        let u32_1 = DynTpProperty::from(DynTpPrimitive::from(1u32));

        // Compare same container type, same primitive type, same val
        assert_eq!(u16_1, 1u16);
        assert_eq!(1u16, u16_1);
        assert_eq!(u32_1, u32_1.clone());

        // Compare different container type, same primitive type, same val
        assert_ne!(u16_1, vec![1u16]);
        assert_ne!(vec![1u16], u16_1);

        // Compare same container type, different primitive type, same val
        assert_ne!(u32_1, 1u16);
        assert_ne!(1u16, u32_1);

        // Compare different container type, different primitive type, same val
        assert_ne!(u32_1, vec![1u16]);
        assert_ne!(vec![1u16], u32_1);

        // compare different container type, different primitive, different val
        assert_ne!(u16_1, vec![2u32])
    }
}
