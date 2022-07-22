// Teleportal Platform v3
// Copyright 2021 WiTag Inc. dba Teleportal

use crate::contract::properties::channels::{
    apply_to_channel, apply_to_channel_id, Channel, ChannelArenaHandle, ChannelArenaMap,
    ChannelHandle, ChannelId, ChannelsIter, DynChannel, IChannels,
};
use crate::contract::properties::dynamic::{apply_to_prop, DynTpProperty};
use crate::contract::properties::states::{
    apply_to_state_id, IStateHandle, IStates, State, StateArenaHandle, StateArenaMap, StateHandle,
    StateId, StatesIter,
};

use crate::contract::properties::traits::ITpPropertyStatic;
use crate::contract::{Contract, ContractData, ContractDataHandle};
use crate::object::{Object, ObjectHandle};
use crate::time::TimeWarp;

use arena::Arena;
use eyre::{eyre, Result};
use itertools::EitherOrBoth;
use itertools::Itertools;

#[cfg(feature = "safer-ffi")]
use safer_ffi::derive_ReprC;

#[cfg_attr(feature = "safer-ffi", derive_ReprC)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum BaselineKind {
    Main = 0,
    Fork,
}

#[cfg_attr(feature = "safer-ffi", derive_ReprC, ReprC::opaque)]
pub struct Baseline {
    kind: BaselineKind,
    objects: Arena<Object>,
    contracts: Arena<ContractData>,
    pub(crate) states: StateArenaMap, // maps from T to Arena<State<T>>
    pub(crate) channels: ChannelArenaMap, // maps from T to Arena<Channel<T>>
}

impl Baseline {
    pub fn new(kind: BaselineKind) -> Self {
        let objects = Arena::new();
        let contracts = Arena::new();
        let states = StateArenaMap::new();
        let channels = ChannelArenaMap::new();

        Self {
            kind,
            objects,
            contracts,
            states,
            channels,
        }
    }

    pub fn kind(&self) -> BaselineKind {
        self.kind
    }

    // ---- Object and Contract Acessors ----

    pub fn register_contract<C: Contract>(&mut self) -> Result<C> {
        for (_, c_data) in self.contracts.iter() {
            let c_id = c_data.id();
            if c_id == C::ID {
                return Err(eyre!("Contract already added!"));
            }
        }
        let handle = self.contracts.insert(ContractData::new(C::ID));
        Ok(C::new(handle))
    }

    pub fn unregister_contract<C: Contract>(&mut self, handle: ContractDataHandle) -> Result<()> {
        let c_data = self
            .contracts
            .get_mut(handle)
            .ok_or_else(|| eyre!("There is no contract with that id to unregister!"))?;

        if c_data.id() != C::ID {
            return Err(eyre!("Handle did not match the provided contract type!"));
        }

        // Its ok to steal the hashmap because c_data will be deleted soon anyway
        let objs = std::mem::take(c_data.objects_mut());
        for o in objs {
            self.object_remove::<C>(o)
                .expect("Failed to remove object!")
        }
        self.contracts.remove(handle);
        Ok(())
    }

    pub fn contract_data(&self, handle: ContractDataHandle) -> Result<&ContractData> {
        self.contracts
            .get(handle)
            .ok_or_else(|| eyre!("No contract exists for that handle!"))
    }

    pub fn iter_objects(&self) -> impl Iterator<Item = (ObjectHandle, &Object)> {
        self.objects.iter()
    }

    pub fn object(&self, obj: ObjectHandle) -> Result<&Object> {
        self.objects
            .get(obj)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn object_mut(&mut self, obj: ObjectHandle) -> Result<&mut Object> {
        self.objects
            .get_mut(obj)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    /// Create an object with the given `states` and `channels`, corresponding
    /// to contract `C`
    ///
    /// # Errors
    /// Will error if the types of any of the states and channels don't match
    /// the contract.
    pub fn object_create<C: Contract>(
        &mut self,
        contract: &C,
        states: impl Iterator<Item = DynTpProperty>,
        channels: impl Iterator<Item = DynChannel>,
    ) -> Result<ObjectHandle> {
        if !self.contracts.contains(contract.handle()) {
            return Err(eyre!("No such contract for that handle"));
        }

        let state_types = C::States::enumerate_types();
        let channel_types = C::Channels::enumerate_types();

        // Check that all types match before attempting to create properties
        macro_rules! check_types {
            ($prop:ident, $types:ident) => {{
                let size = $prop.size_hint().0;
                $prop.zip_longest($types).enumerate().try_fold(
                    Vec::with_capacity(size),
                    |mut acc, (i, either)| {
                        if let EitherOrBoth::Both(p, t) = either {
                            if p.prop_type() != *t {
                                return Err(eyre!(
                                    "Property at field index {} did not match contract type",
                                    i
                                ));
                            }
                            acc.push(p);
                            Ok(acc)
                        } else {
                            return Err(eyre!(
                                "Properties did not match the number of fields in contract"
                            ));
                        }
                    },
                )
            }};
        }

        let states: Vec<DynTpProperty> = check_types!(states, state_types)?;
        let channels: Vec<DynChannel> = check_types!(channels, channel_types)?;

        // actually do the creation
        let mut state_handles: Vec<arena::generational_arena::Index> = Vec::new();
        let mut channel_handles: Vec<arena::generational_arena::Index> = Vec::new();

        for s in states {
            apply_to_prop!(s, |s| state_handles.push(self.state_create(s).into()));
        }
        for c in channels {
            apply_to_channel!(c, |c| channel_handles.push(self.channel_create(c).into()));
        }

        let object = Object::new(
            state_handles,
            channel_handles,
            contract.handle(),
            TimeWarp::default(),
        );
        let obj_handle = self.objects.insert(object);
        self.contracts
            .get_mut(contract.handle())
            .expect("We already checked this")
            .objects_mut()
            .insert(obj_handle);
        Ok(obj_handle)
    }

    pub fn object_remove<C: Contract>(&mut self, obj: ObjectHandle) -> Result<()> {
        let o = if let Some(o) = self.objects.remove(obj) {
            o
        } else {
            return Err(eyre!("Object did not exist, so it could not be removed"));
        };

        // remove all fields of the object
        let states = StatesIter::<C::States>::new(o.contract());
        let channels = ChannelsIter::<C::Channels>::new(o.contract());

        for s in states {
            apply_to_state_id!(s, |id| {
                let handle = self.bind_state(id, obj)?;
                if let Err(e) = self.state_remove(handle) {
                    log::warn!("Failed to remove state, state has been leaked: {}", e);
                }
                Ok::<(), eyre::Report>(())
            })?;
        }

        for c in channels {
            apply_to_channel_id!(c, |id| {
                let handle = self.bind_channel(id, obj)?;
                if let Err(e) = self.channel_remove(handle) {
                    log::warn!("Failed to remove channel, channel has been leaked: {}", e);
                }
                Ok::<(), eyre::Report>(())
            })?;
        }

        Ok(())
    }

    // ---- Property accessors ----

    pub fn state<H: IStateHandle>(&self, state: H) -> Result<H::OutputRef<'_>> {
        state.get(self)
    }

    pub fn state_mut<H: IStateHandle>(&mut self, state: H) -> Result<H::OutputMut<'_>> {
        state.get_mut(self)
    }

    pub fn channel<T: ITpPropertyStatic>(&self, chan: ChannelHandle<T>) -> Result<&Channel<T>> {
        let arena = self
            .channels
            .get()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get(chan)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    pub fn channel_mut<T: ITpPropertyStatic>(
        &mut self,
        chan: ChannelHandle<T>,
    ) -> Result<&mut Channel<T>> {
        let arena = self
            .channels
            .get_mut()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .get_mut(chan)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn state_remove<T: ITpPropertyStatic>(&mut self, state: StateHandle<T>) -> Result<State<T>> {
        let arena = self
            .states
            .get_mut()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .remove(state)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn channel_remove<T: ITpPropertyStatic>(
        &mut self,
        channel: ChannelHandle<T>,
    ) -> Result<Channel<T>> {
        let arena = self
            .channels
            .get_mut()
            .ok_or_else(|| eyre!("The given handle doesn't have an associated Arena"))?;

        arena
            .remove(channel)
            .ok_or_else(|| eyre!("The given handle doesn't exist in the Arena"))
    }

    fn state_create<T: ITpPropertyStatic>(&mut self, value: T) -> StateHandle<T> {
        let arena = self
            .states
            .0
            .entry::<StateArenaHandle<T>>()
            .or_insert_with(|| Arena::new());

        arena.insert(State::new(value))
    }

    fn channel_create<T: ITpPropertyStatic>(&mut self, channel: Channel<T>) -> ChannelHandle<T> {
        let arena = self
            .channels
            .0
            .entry::<ChannelArenaHandle<T>>()
            .or_insert_with(|| Arena::new());

        arena.insert(channel)
    }

    // ---- State and Channel bindings ----

    pub fn bind_state<T: ITpPropertyStatic>(
        &self,
        id: StateId<T>,
        obj: ObjectHandle,
    ) -> Result<StateHandle<T>> {
        let obj = self
            .objects
            .get(obj)
            .ok_or_else(|| eyre!("The given ObjectHandle doesn't exist in the Arena"))?;
        obj.bind_state(id)
    }

    pub fn bind_channel<T: ITpPropertyStatic>(
        &self,
        id: ChannelId<T>,
        obj: ObjectHandle,
    ) -> Result<ChannelHandle<T>> {
        let obj = self
            .objects
            .get(obj)
            .ok_or_else(|| eyre!("The given ObjectHandle doesn't exist in the Arena"))?;
        obj.bind_channel(id)
    }
}

// ---- Index traits ----

impl core::ops::Index<ObjectHandle> for Baseline {
    type Output = Object;

    fn index(&self, index: ObjectHandle) -> &Self::Output {
        &self.objects[index]
    }
}
impl core::ops::IndexMut<ObjectHandle> for Baseline {
    fn index_mut(&mut self, index: ObjectHandle) -> &mut Self::Output {
        &mut self.objects[index]
    }
}

impl<T: ITpPropertyStatic> core::ops::Index<StateHandle<T>> for Baseline {
    type Output = State<T>;

    fn index(&self, index: StateHandle<T>) -> &Self::Output {
        self.state(index).expect("Invalid handle")
    }
}
impl<T: ITpPropertyStatic> core::ops::IndexMut<StateHandle<T>> for Baseline {
    fn index_mut(&mut self, index: StateHandle<T>) -> &mut Self::Output {
        self.state_mut(index).expect("Invalid handle")
    }
}

impl<T: ITpPropertyStatic> core::ops::Index<ChannelHandle<T>> for Baseline {
    type Output = Channel<T>;

    fn index(&self, index: ChannelHandle<T>) -> &Self::Output {
        self.channel(index).expect("Invalid handle")
    }
}
impl<T: ITpPropertyStatic> core::ops::IndexMut<ChannelHandle<T>> for Baseline {
    fn index_mut(&mut self, index: ChannelHandle<T>) -> &mut Self::Output {
        self.channel_mut(index).expect("Invalid handle")
    }
}

#[cfg(feature = "c_api")]
#[rsharp::substitute("tp_client::baseline")]
pub mod c_api {
    #![allow(non_camel_case_types, non_snake_case, dead_code)]

    use super::*;
    use crate::contract::c_api::ContractDataHandle as CContractDataHandle;
    use crate::contract::c_api::ExampleContract;
    use crate::contract::properties::c_api::simple_primitives;
    use crate::object::c_api::ObjectHandle as CObjectHandle;

    use rsharp::remangle;
    use safer_ffi::prelude::*;

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__new(kind: BaselineKind) -> repr_c::Box<Baseline> {
        Box::new(Baseline::new(kind)).into()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__drop(baseline: repr_c::Box<Baseline>) {
        drop(baseline)
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__kind(b: &Baseline) -> BaselineKind {
        b.kind()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__contract_data<'a>(
        b: &'a Baseline,
        contract: &CContractDataHandle,
    ) -> &'a ContractData {
        b.contract_data(contract.inner).unwrap()
    }

    // TODO: iter_objects

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__object<'a>(baseline: &'a Baseline, handle: &CObjectHandle) -> &'a Object {
        baseline.object(handle.inner).unwrap()
    }

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__object_mut<'a>(
        baseline: &'a mut Baseline,
        handle: &CObjectHandle,
    ) -> &'a mut Object {
        baseline.object_mut(handle.inner).unwrap()
    }

    macro_rules! monomorphize {
        // Base case
        ($path:literal, $t:ty $(,)?) => {
            paste::paste! {
                mod [<_Baseline_ $t:camel>] {
                    use super::*;

                    use crate::contract::properties::states::c_api::[<StateHandle_ $t:camel>] as Monomorphized_StateHandle;
                    use crate::contract::properties::states::c_api::[<State_ $t:camel>] as Monomorphized_State;

                    use crate::contract::properties::channels::c_api::[<ChannelHandle_ $t:camel>] as Monomorphized_ChannelHandle;
                    use crate::contract::properties::channels::c_api::[<Channel_ $t:camel>] as Monomorphized_Channel;

                    use crate::contract::properties::states::c_api::[<StateId_ $t:camel>] as Monomorphized_StateId;
                    use crate::contract::properties::channels::c_api::[<ChannelId_ $t:camel>] as Monomorphized_ChannelId;

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Baseline__state_ $t:camel>]<'a>(
                        b: &'a Baseline,
                        state: &Monomorphized_StateHandle
                    ) -> &'a Monomorphized_State {
                        let s: &'a State<$t> = b.state(state.inner).unwrap();
                        s.into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Baseline__state_mut_ $t:camel>]<'a>(
                        b: &'a mut Baseline,
                        state: &Monomorphized_StateHandle
                    ) -> &'a mut Monomorphized_State {
                        let s: &'a mut State<$t> = b.state_mut(state.inner).unwrap();
                        s.into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Baseline__channel_ $t:camel>]<'a>(
                        b: &'a Baseline,
                        chan: &Monomorphized_ChannelHandle
                    ) -> &'a Monomorphized_Channel {
                        let c: &'a Channel<$t> = b.channel(chan.inner).unwrap();
                        c.into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Baseline__channel_mut_ $t:camel>]<'a>(
                        b: &'a mut Baseline,
                        chan: &Monomorphized_ChannelHandle
                    ) -> &'a mut Monomorphized_Channel {
                        let c: &'a mut Channel<$t> = b.channel_mut(chan.inner).unwrap();
                        c.into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Baseline__bind_state_ $t:camel>](
                        b: &Baseline,
                        id: &Monomorphized_StateId,
                        obj: &CObjectHandle,
                    ) -> repr_c::Box<Monomorphized_StateHandle> {
                        let h: StateHandle<$t> = b.bind_state(id.inner, obj.inner).unwrap();
                        Box::new(Monomorphized_StateHandle::from(h)).into()
                    }

                    #[remangle($path)]
                    #[ffi_export]
                    pub fn [<Baseline__bind_channel_ $t:camel>](
                        b: &Baseline,
                        id: &Monomorphized_ChannelId,
                        obj: &CObjectHandle,
                    ) -> repr_c::Box<Monomorphized_ChannelHandle> {
                        let h: ChannelHandle<$t> = b.bind_channel(id.inner, obj.inner).unwrap();
                        Box::new(Monomorphized_ChannelHandle::from(h)).into()
                    }
                }
            }
        };
        // recursive case
        ($path:literal, $first_t:ty, $($tail_t:ty),+ $(,)?) => {
            monomorphize!($path, $first_t);
            monomorphize!($path, $($tail_t),+);
        };
    }

    // This is like doing `monomorphize!("whatever", Keyframe, u8, u16, ...)
    simple_primitives!(; types, monomorphize, "tp_client::baseline");

    // ---- Example contract for tests ---- //

    #[remangle(substitute!())]
    #[ffi_export]
    pub fn Baseline__register_contract___example<'a>(
        baseline: &'a mut Baseline,
    ) -> repr_c::Box<ExampleContract> {
        let c: ExampleContract = baseline
            .register_contract()
            .expect("Failed to register contract");
        Box::new(c).into()
    }
}
