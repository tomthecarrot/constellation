use super::{private, DynTpData, ITpData, ITpProperty, TpDataType};

use crate::contract::ContractId;
use crate::object::ObjectHandle;

use derive_more::From;

/// The type of the ITpProperty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpPropertyType {
    Vec(TpDataType),
    Single(TpDataType),
    Dynamic(TpDataType),
}

/// A dynamically typed container for one or many `T: ITpData`.
#[derive(Debug, Clone, PartialEq, PartialOrd, From)]
pub enum DynTpProperty<T: ITpData = DynTpData> {
    Vec(Vec<T>),
    Single(T),
}

impl<T: ITpData> ITpProperty for DynTpProperty<T> {
    type Data = T;

    const PROPERTY_TYPE: TpPropertyType = TpPropertyType::Dynamic(T::DATA_TYPE);
}

// ---- PartialEq and PartialOrd impls ----

macro_rules! impl_dyntpproperty {
    // base case
    ($t:ty) => {
        // ---- Single variants ----
        impl PartialEq<$t> for DynTpProperty<$t> {
            fn eq(&self, other: &$t) -> bool {
                if let Self::Single(inner) = self {
                    inner == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<DynTpProperty<$t>> for $t {
            fn eq(&self, other: &DynTpProperty<$t>) -> bool {
                if let DynTpProperty::Single(other) = other {
                    self == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<$t> for DynTpProperty {
            fn eq(&self, other: &$t) -> bool {
                if let Self::Single(inner) = self {
                    inner == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<DynTpProperty> for $t {
            fn eq(&self, other: &DynTpProperty) -> bool {
                if let DynTpProperty::Single(other) = other {
                    self == other
                } else {
                    false
                }
            }
        }

        // ---- Vec variants ----

        impl PartialEq<Vec<$t>> for DynTpProperty<$t> {
            fn eq(&self, other: &Vec<$t>) -> bool {
                if let Self::Vec(inner) = self {
                    inner == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<DynTpProperty<$t>> for Vec<$t> {
            fn eq(&self, other: &DynTpProperty<$t>) -> bool {
                if let DynTpProperty::Vec(other) = other {
                    self == other
                } else {
                    false
                }
            }
        }

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
        impl_dyntpproperty!($t);
        impl_dyntpproperty!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_dyntpproperty!($($tail),+);
    };
}

impl_dyntpproperty!(
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
    ContractId,
);

impl private::Sealed for DynTpProperty {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partialeq() {
        let u16_1 = DynTpProperty::from(1u16);
        let dyn_u32_1 = DynTpProperty::from(DynTpData::from(1u32));

        // Compare same container type & value
        assert_eq!(u16_1, 1u16);
        assert_eq!(1u16, u16_1);
        assert_eq!(dyn_u32_1, dyn_u32_1.clone());

        // Compare mismatched container type, same val
        assert_ne!(u16_1, vec![1u16]);
        assert_ne!(vec![1u16], u16_1);

        // Compare same container type, different dyn scalar type, same val
        assert_ne!(dyn_u32_1, 1u16);
        assert_ne!(1u16, dyn_u32_1);

        // Compare different container type, different dyn scalar type, same val
        assert_ne!(dyn_u32_1, vec![1u16]);
        assert_ne!(vec![1u16], dyn_u32_1);
    }
}
