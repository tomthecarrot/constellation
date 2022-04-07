use super::dyn_channel::{DynChannelMut, DynChannelRef};
use super::{handle::ChannelHandle, IChannelHandle};
use crate::apply_to_channel_handle;
use crate::contract::properties::channels::Channel;
use crate::contract::properties::dynamic::__macro::DynEnum;
use crate::contract::properties::dynamic::{TpPrimitiveType, TpPropertyType};
use crate::contract::properties::primitives;
use crate::contract::properties::traits::ITpPropertyStatic;

DynEnum!(DynChannelHandle, ChannelHandle | derive(Clone, PartialEq));

impl Copy for DynChannelHandlePrimitive {}
impl Copy for DynChannelHandleVec {}
impl Copy for DynChannelHandle {}

impl IChannelHandle for DynChannelHandle {
    type OutputRef<'a> = DynChannelRef<'a>;
    type OutputMut<'a> = DynChannelMut<'a>;

    fn get<'a>(
        &self,
        baseline: &'a crate::baseline::Baseline,
    ) -> eyre::Result<Self::OutputRef<'a>> {
        // Matches on the type and then calls the appropriate generic function in baseline
        apply_to_channel_handle!(*self, |h: ChannelHandle<_>| {
            let chan = h.get(baseline)?;
            Ok(DynChannelRef::from(chan))
        })
    }

    fn get_mut<'a>(
        &self,
        baseline: &'a mut crate::baseline::Baseline,
    ) -> eyre::Result<Self::OutputMut<'a>> {
        fn make_dyn<'a, T>(
            h: ChannelHandle<T>,
            baseline: &'a mut crate::baseline::Baseline,
        ) -> eyre::Result<DynChannelMut<'a>>
        where
            T: ITpPropertyStatic,
            DynChannelMut<'a>: From<&'a mut Channel<T>>,
        {
            let chan = h.get_mut(baseline)?;
            Ok(DynChannelMut::from(chan))
        }

        // Not sure why this can't just use `apply_to_channel_handle!()`, but it
        // can't figure out the lifetime inference with that approach. So we match
        // with an enum + macro instead.
        macro_rules! match_helper {
            ($enum_ident:ident, $enum_type:ident, $($variant:ident),+ $(,)?) => {
                match $enum_ident {
                    $($enum_type::$variant(h) => make_dyn(h, baseline)),*
                }
            }
        }

        match *self {
            DynChannelHandle::Primitive(p) => {
                primitives!(idents, match_helper, p, DynChannelHandlePrimitive)
            }
            DynChannelHandle::Vec(v) => primitives!(idents, match_helper, v, DynChannelHandleVec),
        }
    }

    fn prop_type(&self) -> TpPropertyType {
        use DynChannelHandlePrimitive as P;
        use DynChannelHandleVec as V;

        // Maps enum to `TpPrimitiveType` by expanding to match on variants
        macro_rules! helper_match {
            ($enum:ident, $enum_type:ident, $($ident:ident),+ $(,)?) => {
                match $enum {
                    $(
                        $enum_type::$ident(_) => TpPrimitiveType::$ident,
                    )*
                }
            };
        }

        match self {
            Self::Primitive(p) => {
                TpPropertyType::Primitive(primitives!(idents, helper_match, p, P))
            }
            Self::Vec(v) => TpPropertyType::Vec(primitives!(idents, helper_match, v, V)),
        }
    }
}
