use crate::serializer::handle_map::{ContractsIdx, HandleMap};

use crate::{c, t};
use eyre::{eyre, Result, WrapErr};
use tp_client::contract::properties::states::IStates;

pub struct Deserializer<'a> {
    data: &'a [u8],
    baseline: c::Baseline,
    handle_map: HandleMap,
}

impl<'a> Deserializer<'a> {
    pub fn deserialize<C: c::Contract>(&mut self) -> Result<C> {
        self.handle_map.contracts.
        let baseline_t = flatbuffers::root::<t::Baseline>(self.data)
            .wrap_err("Failed to get baseline root flatbuffer")?;
        let (c_idx, c_t) = find_contract::<C>(baseline_t).wrap_err("Failed to find contract")?;
        todo!()
    }
    pub fn finish(self) -> c::Baseline {
        todo!()
    }
}

/// Validates that `contract_t` matches all the properties in `C`.
fn find_contract<'a, C: c::Contract>(
    baseline_t: t::Baseline<'a>,
) -> eyre::Result<(ContractsIdx, t::Contract<'_>)> {
    let Some(contracts_t) = baseline_t.contracts() else {
        return Err(eyre!("There are no contracts to deserialize"));
    };

    // Deserialization would be faster if we searched for *all* contracts we
    // wanted to deserialize here, and not just an O(n) search for a single one.
    // But I'm punting this optimization until we know we need it.
    contracts_t
        .into_iter()
        .enumerate()
        .find(|(_idx, c)| {
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
                let types = states_t.types()?;
                // Lengths match?
                (names.len() == nfields && types.len() == nfields).then_some(())?;
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
        .ok_or(eyre!("Coult not find a matching contract!"))
        .map(|(idx, c)| (ContractsIdx(idx), c))
}
