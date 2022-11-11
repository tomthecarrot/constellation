pub(crate) mod handle_map;

use eyre::{eyre, Result, WrapErr};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use paste::paste;
use tp_client::apply_to_state_id;
use tp_client::contract::properties::dynamic::{DynTpPrimitiveRef, DynTpPropertyRef};
use tp_client::contract::properties::states::dyn_handle::DynStateHandlePrimitive;
use tp_client::contract::properties::states::dyn_state::DynStateRef;
use tp_client::contract::properties::states::{DynStateHandle, IStates};

use self::handle_map::{ContractsIdx, HandleMap, ObjectsIdx, StatesIdx};
use crate::baseline::BaselineArgs;
use crate::contract::{ContractArgs, ContractDataHandleArgs, ContractIdArgs, ContractStatesArgs};
use crate::object::{ObjectArgs, ObjectHandleArgs};
use crate::primitive::FbStringArgs;
use crate::state::{StateArgs, StateHandleArgs};
use crate::{c, t};

/// The dummy value that will be used for temporary `WIPOffset` values.
const WIP_DUMMY: flatbuffers::UOffsetT = 1337;

pub struct Serializer<'b> {
    fbb: FlatBufferBuilder<'static>,
    baseline: &'b c::Baseline,
    contracts: Vec<WIPOffset<t::Contract<'static>>>,
    states: Vec<WIPOffset<t::State<'static>>>,
    objects: Vec<WIPOffset<t::Object<'static>>>,
    handle_map: HandleMap,
}
impl<'b> Serializer<'b> {
    pub fn new(mut fbb: FlatBufferBuilder<'static>, baseline: &'b c::Baseline) -> Serializer<'b> {
        fbb.reset();
        Self {
            fbb,
            baseline,
            contracts: Vec::new(),
            states: Vec::new(),
            objects: Vec::new(),
            handle_map: Default::default(),
        }
    }

    /// Serialize all objects related to contract `C`. Usually, this gets called once
    /// per relevant contract.
    pub fn serialize<C: c::Contract>(&mut self, contract: &C) -> Result<()> {
        let fbb = &mut self.fbb;
        let contract_data: &c::ContractData = self.baseline.contract_data(contract.handle())?;

        self.contracts.push(Self::serialize_contract::<C>(fbb)?);
        self.handle_map
            .insert_contract(contract.handle(), ContractsIdx(self.contracts.len() - 1));

        let contract_data_handle_t = {
            let idx = self.contracts.len() as u16 - 1;
            t::ContractDataHandle::create(fbb, &ContractDataHandleArgs { idx })
        };

        for &obj_handle in contract_data.objects().iter() {
            let mut state_handles = Vec::new();
            for state_id in contract.state_iter() {
                let (state_handle, state) =
                    apply_to_state_id!(state_id, |state_id| -> eyre::Result<_> {
                        let state_handle = self
                            .baseline
                            .bind_state(state_id, obj_handle)
                            .wrap_err("Failed to bind StateId to Object")?;
                        let state = DynStateRef::from(self.baseline.state(state_handle)?);
                        let state_handle = DynStateHandle::from(state_handle);
                        Ok((state_handle, state))
                    })?;

                macro_rules! helper {
                    ($t:ident, $v:expr) => {{
                        paste! {
                            let p = t::primitive::$t::create(fbb, &$crate::primitive::[<$t Args>] { v: $v });
                            t::State::create(
                                fbb,
                                &StateArgs {
                                    p_type: t::TpPrimitive::$t,
                                    p: Some(p.as_union_value()),
                                },
                            )
                        }
                    }};
                }
                let state_t: WIPOffset<t::State> = match state.0 {
                    DynTpPropertyRef::Primitive(p) => match p {
                        DynTpPrimitiveRef::U8(&p) => helper!(U8, p),
                        DynTpPrimitiveRef::U16(&p) => helper!(U16, p),
                        DynTpPrimitiveRef::U32(&p) => helper!(U32, p),
                        DynTpPrimitiveRef::U64(&p) => helper!(U64, p),
                        DynTpPrimitiveRef::I8(&p) => helper!(I8, p),
                        DynTpPrimitiveRef::I16(&p) => helper!(I16, p),
                        DynTpPrimitiveRef::I32(&p) => helper!(I32, p),
                        DynTpPrimitiveRef::I64(&p) => helper!(I64, p),
                        DynTpPrimitiveRef::Bool(&p) => helper!(Bool, p),
                        DynTpPrimitiveRef::F32(&p) => helper!(F32, p),
                        DynTpPrimitiveRef::F64(&p) => helper!(F64, p),
                        DynTpPrimitiveRef::String(s) => {
                            let s = fbb.create_string(s.as_str());
                            let p =
                                t::primitive::FbString::create(fbb, &FbStringArgs { v: Some(s) });
                            t::State::create(
                                fbb,
                                &StateArgs {
                                    p_type: t::TpPrimitive::FbString,
                                    p: Some(p.as_union_value()),
                                },
                            )
                        }
                        // Handles will be serialized to a dummy value, and populated later
                        DynTpPrimitiveRef::ObjectHandle(_)
                        | DynTpPrimitiveRef::ContractDataHandle(_) => {
                            WIPOffset::<t::State>::new(WIP_DUMMY)
                        }
                    },
                    DynTpPropertyRef::Vec(_v) => todo!("Vectors not supported yet"),
                };
                self.states.push(state_t);
                let idx = self.states.len() - 1;
                // Insert state handles
                match state_handle {
                    DynStateHandle::Primitive(DynStateHandlePrimitive::ObjectHandle(h)) => {
                        self.handle_map.insert_object_state(h, StatesIdx(idx));
                    }
                    DynStateHandle::Primitive(DynStateHandlePrimitive::ContractDataHandle(h)) => {
                        self.handle_map.insert_contract_state(h, StatesIdx(idx));
                    }
                    _ => (), // Do nothing for non-handles
                }

                let state_handle_t =
                    t::StateHandle::create(fbb, &StateHandleArgs { idx: idx as u32 });
                state_handles.push(state_handle_t);
            }

            let state_handles_t = fbb.create_vector_from_iter(state_handles.into_iter());
            let obj_t = t::Object::create(
                fbb,
                &ObjectArgs {
                    contract: Some(contract_data_handle_t),
                    states: Some(state_handles_t),
                },
            );
            self.objects.push(obj_t);
            self.handle_map
                .insert_object(obj_handle, ObjectsIdx(self.objects.len() - 1));
        }
        Ok(())
    }

    fn serialize_contract<C: c::Contract>(
        fbb: &mut FlatBufferBuilder<'static>,
    ) -> Result<WIPOffset<t::Contract<'static>>> {
        let cid_t = {
            let (v_major, v_minor, v_patch) = C::ID.version;
            let name_t = fbb.create_string(C::ID.name);
            t::ContractId::create(
                fbb,
                &ContractIdArgs {
                    name: Some(name_t),
                    v_major,
                    v_minor,
                    v_patch,
                },
            )
        };
        let cstates_t = {
            let names_t = {
                let names_t: Vec<_> = C::States::field_names()
                    .into_iter()
                    .map(|n| fbb.create_string(n))
                    .collect();
                fbb.create_vector(&names_t)
            };
            let types_t = {
                let types_t: Result<Vec<_>> = C::States::enumerate_types()
                    .into_iter()
                    .map(|t| match t {
                        c::TpPropertyType::Primitive(p) => Ok(t::TpPrimitiveKind::from(*p)),
                        c::TpPropertyType::Vec(_v) => {
                            return Err(eyre!("Vectors are not yet supported"))
                        }
                    })
                    .collect();
                let types_t = types_t?;
                fbb.create_vector(&types_t)
            };
            t::ContractStates::create(
                fbb,
                &ContractStatesArgs {
                    names: Some(names_t),
                    types: Some(types_t),
                },
            )
        };
        Ok(t::Contract::create(
            fbb,
            &ContractArgs {
                id: Some(cid_t),
                states: Some(cstates_t),
            },
        ))
    }

    pub fn finish(mut self) -> FlatBufferBuilder<'static> {
        // Second pass to write the handle data. We will go back through the handles and
        // properly update their indices by using the `HandleMap`
        macro_rules! rewrite_states {
            ($handle_ident:ident, $map_field:expr, $handle_variant:expr $(,)?) => {
                for (state_handle, state_idx) in $map_field.iter() {
                    let referenced_idx = {
                        let referenced_handle = self
                            .baseline
                            .state(*state_handle)
                            .expect("Unexpectly had a missing handle")
                            .value;
                        self.handle_map[referenced_handle]
                    };

                    // Create the state flatbuffer
                    let state_offset = paste::paste! {{
                        let p = t::$handle_ident::create(
                            &mut self.fbb,
                            &[<$handle_ident Args>] {
                                idx: referenced_idx.0 as _,
                            },
                        );
                        t::State::create(
                            &mut self.fbb,
                            &StateArgs {
                                p_type: $handle_variant,
                                p: Some(p.as_union_value()),
                            },
                        )
                    }};
                    debug_assert_eq!(self.states[state_idx.0].value(), WIP_DUMMY);
                    self.states[state_idx.0] = state_offset;
                }
            };
        }
        rewrite_states!(
            ContractDataHandle,
            self.handle_map.contract_states,
            t::TpPrimitive::tp_serialize_contract_ContractDataHandle,
        );
        rewrite_states!(
            ObjectHandle,
            self.handle_map.object_states,
            t::TpPrimitive::tp_serialize_object_ObjectHandle,
        );

        // Now we need to actually serialize all of these vectors into the final buffer
        let fbb = &mut self.fbb;
        let baseline_t = {
            let contracts_t = fbb.create_vector_from_iter(self.contracts.into_iter());
            let states_t = fbb.create_vector_from_iter(self.states.into_iter());
            let objects_t = fbb.create_vector_from_iter(self.objects.into_iter());
            t::Baseline::create(
                fbb,
                &BaselineArgs {
                    contracts: Some(contracts_t),
                    states: Some(states_t),
                    objects: Some(objects_t),
                },
            )
        };
        fbb.finish(baseline_t, Some(crate::PREFIX));
        self.fbb
    }
}
