use eyre::{eyre, Result, WrapErr};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use tp_client::apply_to_state_id;
use tp_client::contract::properties::states::IStates;
use tp_client::contract::properties::states::dyn_state::DynStateRef;

use crate::baseline::BaselineArgs;
use crate::contract::{ContractArgs, ContractIdArgs, ContractStatesArgs};
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

        // TODO: serialize states and objects
        // let mut state_handles = Vec::new();
        for &obj_handle in contract_data.objects().iter() {
            // state_handles.clear();
            for state_id in contract.state_iter() {
                let state: DynStateRef = apply_to_state_id!(state_id, |state_id| {
                    self.baseline
                        .bind_state(state_id, obj_handle)
                        .wrap_err("Failed to bind StateId to Object")
                        .and_then(|h| self.baseline.state(h))
                        .map(DynStateRef::from)
                })?;
            }
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
                        c::TpPropertyType::Vec(v) => {
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
