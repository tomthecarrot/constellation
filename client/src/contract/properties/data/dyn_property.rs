use super::{private, DynTpData, ITpData, ITpProperty};

use crate::contract::ContractHandle;
use crate::object::ObjectHandle;

use derive_more::From;

/// A dynamically typed container for one or many `T: ITpData`.
#[derive(Debug, Clone, PartialEq, PartialOrd, From)]
pub enum DynTpProperty<T: ITpData = DynTpData> {
    Vec(Vec<T>),
    Single(T),
}

impl<T: ITpData> ITpProperty for DynTpProperty<T> {
    type Data = T;
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

        impl PartialOrd<$t> for DynTpProperty<$t> {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                match self {
                    Self::Single(inner) => inner.partial_cmp(other),
                    _ => None,
                }
            }
        }

        impl PartialOrd<DynTpProperty<$t>> for $t {
            fn partial_cmp(&self, other: &DynTpProperty<$t>) -> Option<std::cmp::Ordering> {
                match other {
                    DynTpProperty::Single(other) => self.partial_cmp(other),
                    _ => None,
                }
            }
        }

        impl PartialOrd<$t> for DynTpProperty {
            fn partial_cmp(&self, other: &$t) -> Option<std::cmp::Ordering> {
                match self {
                    Self::Single(inner) => inner.partial_cmp(other),
                    _ => None,
                }
            }
        }

        impl PartialOrd<DynTpProperty> for $t {
            fn partial_cmp(&self, other: &DynTpProperty) -> Option<std::cmp::Ordering> {
                match other {
                    DynTpProperty::Single(other) => self.partial_cmp(other),
                    _ => None,
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

        impl PartialOrd<Vec<$t>> for DynTpProperty<$t> {
            fn partial_cmp(&self, other: &Vec<$t>) -> Option<std::cmp::Ordering> {
                match self {
                    Self::Vec(inner) => inner.partial_cmp(other),
                    _ => None,
                }
            }
        }

        impl PartialOrd<DynTpProperty<$t>> for Vec<$t> {
            fn partial_cmp(&self, other: &DynTpProperty<$t>) -> Option<std::cmp::Ordering> {
                match other {
                    DynTpProperty::Vec(other) => self.partial_cmp(other),
                    _ => None,
                }
            }
        }

        // TODO: These are not implemented because PartialOrd not implemented on
        // Vec<T> and Vec<U> where T: PartialOrd<U>
        // impl PartialOrd<Vec<$t>> for DynTpProperty
        // impl PartialOrd<DynTpProperty> for Vec<$t>

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
    ContractHandle,
);

impl private::Sealed for DynTpProperty {}

#[cfg(test)]
mod test {
    use std::{cmp::Ordering, vec};

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

    #[test]
    fn test_partialord() {
        let u16_1 = DynTpProperty::from(1u16);
        let dyn_u32_1 = DynTpProperty::from(DynTpData::from(1u32));

        // Compare same container type & value
        assert_eq!(u16_1.partial_cmp(&1u16), Some(Ordering::Equal));
        assert_eq!(1u16.partial_cmp(&u16_1), Some(Ordering::Equal));
        assert_eq!(
            dyn_u32_1.partial_cmp(&dyn_u32_1.clone()),
            Some(Ordering::Equal)
        );
        assert!(u16_1 < 2u16);
        assert!(2u16 > u16_1);
        assert!(u16_1 > 0u16);
        assert!(0u16 < u16_1);

        // Compare mismatched container type, same val
        assert_eq!(u16_1.partial_cmp(&vec![1u16]), None);
        assert_eq!(vec![1u16].partial_cmp(&u16_1), None);

        // Compare same container type, different dyn scalar type, same val
        assert_eq!(dyn_u32_1.partial_cmp(&1u16), None);
        assert_eq!(1u16.partial_cmp(&dyn_u32_1), None);
    }
}
