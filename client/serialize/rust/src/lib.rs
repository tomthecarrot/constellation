mod generated;

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
                            .all(|(a, b)| types_match(*a, b))
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

            for (object_idx, o) in objects_t.into_iter().enumerate() {
                //
            }

            Ok(())
        };
        helper()?;

        Ok(contract)
    }
}

/// Checks if `client` and `table` represent the same `TpPropertyType`
fn types_match(client: c::TpPropertyType, table: primitive::TpPrimitiveKind) -> bool {
    use c::TpPrimitiveType as C;
    use c::TpPropertyType::Primitive;
    use primitive::TpPrimitiveKind as T;
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
