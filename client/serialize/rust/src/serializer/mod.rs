use eyre::{eyre, Result, WrapErr};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use paste::paste;
use constellation::apply_to_state_id;
use constellation::contract::properties::dynamic::{DynTpPrimitiveRef, DynTpPropertyRef};
use constellation::contract::properties::states::dyn_state::DynStateRef;
use constellation::contract::properties::states::IStates;

use crate::baseline::BaselineArgs;
use crate::contract::{ContractArgs, ContractDataHandleArgs, ContractIdArgs, ContractStatesArgs};
use crate::object::ObjectArgs;
use crate::primitive::StringArgs;
use crate::state::{StateArgs, StateHandleArgs};
use crate::{c, t};

pub struct Serializer<'b> {
    fbb: FlatBufferBuilder<'static>,
    baseline: &'b c::Baseline,
    contracts: Vec<WIPOffset<t::Contract<'static>>>,
    states: Vec<WIPOffset<t::State<'static>>>,
    objects: Vec<WIPOffset<t::Object<'static>>>,
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
        }
    }

    /// Serialize all objects related to contract `C`. Usually, this gets called once
    /// per relevant contract.
    pub fn serialize<C: c::Contract>(&mut self, contract: C) -> Result<()> {
        let fbb = &mut self.fbb;
        let contract_data: &c::ContractData = self.baseline.contract_data(contract.handle())?;

        self.contracts.push(Self::serialize_contract::<C>(fbb)?);
        let contract_data_handle_t = {
            let idx = self.contracts.len() as u16 - 1;
            t::ContractDataHandle::create(fbb, &ContractDataHandleArgs { idx })
        };

        for &obj_handle in contract_data.objects().iter() {
            let mut state_handles = Vec::new();
            for state_id in contract.state_iter() {
                let state: DynStateRef = apply_to_state_id!(state_id, |state_id| {
                    self.baseline
                        .bind_state(state_id, obj_handle)
                        .wrap_err("Failed to bind StateId to Object")
                        .and_then(|h| self.baseline.state(h))
                        .map(DynStateRef::from)
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
                            let p = t::primitive::String::create(fbb, &StringArgs { v: Some(s) });
                            t::State::create(
                                fbb,
                                &StateArgs {
                                    p_type: t::TpPrimitive::String,
                                    p: Some(p.as_union_value()),
                                },
                            )
                        }
                        DynTpPrimitiveRef::ObjectHandle(_) => {
                            todo!("SER-464: Figure out how to map handles to fb indices")
                        }
                        DynTpPrimitiveRef::ContractDataHandle(_) => {
                            todo!("SER-464: Figure out how to map handles to fb indices")
                        }
                    },
                    DynTpPropertyRef::Vec(_v) => todo!(),
                };
                self.states.push(state_t);
                let idx = self.states.len() as u32 - 1;
                let state_handle_t = t::StateHandle::create(fbb, &StateHandleArgs { idx });
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
