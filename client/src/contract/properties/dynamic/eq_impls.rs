use super::property::{DynTpPrimitive, DynTpProperty, DynTpVec};
use crate::contract::properties::primitives;
use crate::contract::ContractDataHandle;
use crate::object::ObjectHandle;

// Cannot flip LHS&RHS and do blanket impl because of E0210 (orphan rule)
impl<T> PartialEq<T> for DynTpProperty
where
    DynTpPrimitive: PartialEq<T>,
    T: ?Sized,
{
    fn eq(&self, other: &T) -> bool {
        if let Self::Primitive(p) = self {
            p == other
        } else {
            false
        }
    }
}

impl<'a, T> PartialEq<&'a [T]> for DynTpProperty
where
    DynTpVec: PartialEq<&'a [T]>,
{
    fn eq(&self, other: &&'a [T]) -> bool {
        if let Self::Vec(v) = self {
            v == other
        } else {
            false
        }
    }
}

impl<T> PartialEq<Vec<T>> for DynTpProperty
where
    DynTpVec: PartialEq<Vec<T>>,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        if let Self::Vec(v) = self {
            v == other
        } else {
            false
        }
    }
}

// ---- PartialEq impls with dyn on RHS (cannot do blanket impl on LHS impl due to covered orphan rule E0210) ----

macro_rules! impl_eq {
    // base case
    ($t:ty) => {
        impl PartialEq<DynTpProperty> for $t {
            fn eq(&self, other: &DynTpProperty) -> bool {
                other == self
            }
        }

        impl PartialEq<DynTpProperty> for &[$t] {
            fn eq(&self, other: &DynTpProperty) -> bool {
                other == self
            }
        }

        impl PartialEq<DynTpProperty> for Vec<$t> {
            fn eq(&self, other: &DynTpProperty) -> bool {
                other == self
            }
        }
    };
    // recursive case
    ($t:ty, $($tail:ty),+) => {
        impl_eq!($t);
        impl_eq!($($tail),+);
    };
    // handle trailing comma
    ($($tail:ty),+,) => {
        impl_eq!($($tail),+);
    };
}

primitives!(; types, impl_eq);

#[cfg(test)]
mod tests {
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
