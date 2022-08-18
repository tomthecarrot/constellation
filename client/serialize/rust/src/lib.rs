mod generated;
mod impls;

pub use generated::tp_serialize::*;

use bimap::BiHashMap;
use eyre::{eyre, Result, WrapErr};
use tp_client::contract::properties::states::IStates;

/// The types related to the tp_client
mod c {
    pub use tp_client::baseline::Baseline;
    pub use tp_client::contract::properties::dynamic::{TpPrimitiveType, TpPropertyType};
    pub use tp_client::contract::{Contract, ContractDataHandle, ContractId};
}

/// The types related to the flatbuffer
mod t {
    pub use crate::baseline::Baseline;
}

pub struct Deserializer<'a> {
    data: &'a [u8],
    baseline: c::Baseline,
    contract_idxs: BiHashMap<c::ContractId, usize>,
    contract_data_handles: BiHashMap<c::ContractId, c::ContractDataHandle>,
}
impl<'a> Deserializer<'a> {
    pub fn new(data: &'a [u8], baseline: c::Baseline) -> Self {
        Self {
            data,
            baseline,
            contract_idxs: Default::default(),
            contract_data_handles: Default::default(),
        }
    }

    /// Deserialize all objects related to contract `C` into `baseline`.
    /// Usually, this gets called once per relevant contract
    pub fn deserialize<C: c::Contract>(&mut self) -> Result<C> {
        let baseline_t = flatbuffers::root::<t::Baseline>(self.data)
            .wrap_err("Error while verifying flatbuffer")?;

        // Validate and register the contract
        let contract = {
            let contracts_t = if let Some(c) = baseline_t.contracts() {
                c
            } else {
                return Err(eyre!("There are no contracts to deserialize"));
            };

            debug_assert_eq!(
                self.contract_data_handles.contains_left(&C::ID),
                self.contract_idxs.contains_left(&C::ID),
            );

            if self.contract_idxs.contains_left(&C::ID) {
                return Err(eyre!("Already deserialized contract. ID: {:?}", C::ID));
            }

            // Deserialization would be faster if we searched for *all* contracts we
            // wanted to deserialize here, and not just an O(n) search for a single one.
            // But I'm punting this optimization until we know we need it.
            let (contract_idx, contract_t) = contracts_t
                .into_iter()
                .enumerate()
                .find(|(idx, c)| {
                    // Using option to give us try operator.
                    || -> Option<()> {
                        let id = c.id()?;
                        (id.name()? == C::ID.name
                            && (id.v_major(), id.v_minor(), id.v_patch()) == C::ID.version)
                            .then_some(())
                    }()
                    // Check that properties match
                    .and_then(|_| {
                        let states_t = c.states()?;
                        let nfields = C::States::field_names().len();
                        let names = states_t.names()?;
                        let ids = states_t.ids()?;
                        let types = states_t.types()?;
                        // Lengths match?
                        (names.len() == nfields && ids.len() == nfields && types.len() == nfields)
                            .then_some(())?;
                        // Names match?
                        std::iter::zip(C::States::field_names().into_iter(), names.iter())
                            .all(|(a, b)| *a == b)
                            .then_some(())?;
                        // Types match?
                        std::iter::zip(C::States::enumerate_types().into_iter(), types.iter())
                            .all(|(a, b)| *a == b)
                            .then_some(())?;

                        Some(())
                    })
                    .is_some()
                })
                .ok_or(eyre!("Coult not find a matching contract!"))?;

            let contract: C = self.baseline.register_contract()?;
            self.contract_data_handles.insert(C::ID, contract.handle());
            self.contract_idxs.insert(C::ID, contract_idx);

            contract
        };

        // Create all objects associated with the contract.
        // Uses helper closure just to enable early returns.
        let helper = || -> Result<()> {
            let objects_t = if let Some(o) = baseline_t.objects() {
                o
            } else {
                return Ok(());
            };

            let contract_idx = *self.contract_idxs.get_by_left(&C::ID).unwrap();
            let num_states = C::States::enumerate_types().len();

            objects_t
                .into_iter()
                .enumerate()
                // Filter to just the objects that match our contract
                .filter(|(_idx, obj)| {
                    let c = if let Some(c) = obj.contract() {
                        c
                    } else {
                        return false;
                    };
                    c.idx() as usize == contract_idx
                })
                // create states and object in baseline
                .try_for_each(|(idx, obj_t)| -> Result<()> {
                    // Validate that number of states in object matches contract
                    let obj_states_t = {
                        obj_t.states();
                        if obj_t.states().map_or(0, |x| x.len()) != num_states {
                            return Err(eyre!(
                                "number of states in serialized object did not match contract"
                            ));
                        }
                        if let Some(s) = obj_t.states() {
                            s
                        } else {
                            return Ok(());
                        }
                    };

                    // Go through the object's states and validate that they match the
                    // contract
                    for (i, (obj_state_t, typ)) in
                        std::iter::zip(obj_states_t, C::States::enumerate_types().into_iter())
                            .enumerate()
                    {
                        let states_t = baseline_t
                            .states()
                            .ok_or(eyre!("Expected at least one state!"))?;
                        let state_t = states_t.get(obj_state_t.idx() as usize);
                        if state_t.p_type() != *typ {
                            return Err(eyre!(
                                "state #{i} type was {:?} but expected {:?}",
                                state_t.p_type().variant_name().unwrap(),
                                *typ,
                            ));
                        }
                    }

                    // TODO: Actually create the states
                    Ok(())
                })?;

            Ok(())
        };
        helper()?;

        Ok(contract)
    }
}
