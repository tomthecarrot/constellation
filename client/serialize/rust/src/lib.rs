mod generated;
mod idx_mapper;

use generated::tp_serialize::baseline::Baseline;
pub use generated::tp_serialize::*;

use eyre::{eyre, Result, WrapErr};
use tp_client::contract::properties::states::IStates;
use tp_client::contract::Contract;

/// Deserialize all objects related to contract `C` into `baseline`.
/// Usually, this gets called once per relevant contract
pub fn deserialize<C: Contract>(
    data: &[u8],
    baseline: &mut tp_client::baseline::Baseline,
) -> Result<C> {
    let baseline_t =
        flatbuffers::root::<Baseline>(data).wrap_err("Error while verifying flatbuffer")?;

    let contracts_t = if let Some(c) = baseline_t.contracts() {
        c
    } else {
        return Err(eyre!("There are no contracts to deserialize"));
    };

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
                    .all(|(a, b)| types_match(*a, b))
                    .then_some(())?;

                Some(())
            })
            .is_some()
        })
        .ok_or(eyre!("Coult not find a matching contract!"))?;

    let contract: C = baseline.register_contract()?;

    Ok(todo!())
}

/// Checks if `client` and `table` represent the same `TpPropertyType`
fn types_match(
    client: tp_client::contract::properties::dynamic::TpPropertyType,
    table: primitive::TpPrimitiveKind,
) -> bool {
    use primitive::TpPrimitiveKind as T;
    use tp_client::contract::properties::dynamic::TpPrimitiveType as C;
    use tp_client::contract::properties::dynamic::TpPropertyType::Primitive;
    match (table, client) {
        (T::U8, Primitive(C::U8))
        | (T::U16, Primitive(C::U16))
        | (T::U32, Primitive(C::U32))
        | (T::U64, Primitive(C::U64))
        | (T::I8, Primitive(C::I8))
        | (T::I16, Primitive(C::I16))
        | (T::I32, Primitive(C::I32))
        | (T::I64, Primitive(C::I64))
        | (T::Bool, Primitive(C::Bool))
        | (T::F32, Primitive(C::F32))
        | (T::F64, Primitive(C::F64))
        | (T::String, Primitive(C::String))
        | (T::ObjectHandle, Primitive(C::ObjectHandle))
        | (T::ContractDataHandle, Primitive(C::ContractDataHandle)) => true,
        _ => false,
    }
}
