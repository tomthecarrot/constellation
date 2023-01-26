//! Explanation:
//!
//! States:
//! * Can always be created when they don't hold a handle.
//! * Can be created if its referencing a contract and the contract exists.
//! * Can be created if its referencing an object and the object exists.
//!
//! Object:
//! * Can only be created when all states referenced exist and contract exists.
//!
//! Contract:
//! * Holds no references, so it can always be created.
//!
//! We want to instantiate everything in a reverse topological sort, where we instantiate things
//! that only point to stuff already instantiated. However, there is a catch. In a topolocial sort,
//! there can be no cycles. In our case, we could unfortunately have a cycle where two object's
//! states reference eachother.
//!
//! To work around this, we will create a single "dummy" object belonging to a dummy class with no
//! states. The handle of this dummy object will be used any time we have a `State<ObjectHandle>`,
//! and we will mark that object as needing a second pass to restore the proper object handle.
//!
//! The algorithm to deserialize the flatbuffer into a `Baseline` looks like this:
//!
//! 1. Instantiate all contracts. This is done by the user "registering" each contract they plan on
//!    using to the `Deserializer`. If a contract was serialized but is not provided at this stage,
//!    any objects that reference that contract will not be deserialized, and we will error the
//!    deserialization, notifying the caller. Simultaneously, keep a bidirectional map of the
//!    `ContractDataHandle`s and the contract's index in the flatbuffer. At each registration, we
//!    also return to the caller their instantiated contract, so that they can use it later.
//! 2. We will register an additional "Null" contract
//! 3. Create a single dummy object of that contract, as a "null" object
//! 4. Iterate over every state, and instantiate it in the baseline. `State<ObjectHandle>`s will
//!    use the null object's handle. Simultaneously, keep a bidirectional map of these
//!    `StateHandle`s and the index of the object that the `State<ObjectHandle>` was referencing.
//!    Also keep track of which of these were referencing the "null" object.
//! 5. Iterate over every serialized object. Use the contract map to ensure that its contract was
//!    already deserialized, if not, error. Make sure that every state in the serialized object has
//!    the correct type for its contract. Also, for every state in the object, ensure that it exists in
//!    the serialized flatbuffer by validating that the baseline.states index is in the bounds of
//!    the array. Once everything has been validated, we can instantiate the object in the
//!    baseline, looking up the appropriate state handles from the state map.
//! 6. Iterate over the states that referenced the null object. Have them store the appropriate
//!    `ObjectHandle` instead, by using the mapping from the original serialized object index to the
//!    deserialized `ObjectHandle`.
//! 7. Delete the null contract and its null object.
//! 8. Everything should be deserialized in the baseline now. Return the baseline to the caller.

mod contracts;
mod null_contract;
mod objects;
mod states;

use self::contracts::InstantiatedContracts;
use self::null_contract::NullContract;
use self::objects::InstantiatedObjects;
use self::states::InstantiatedStates;
use crate::types::{ContractsIdx, ObjectsIdx, StatesIdx};
use crate::{fb, rs};

use eyre::{eyre, Result, WrapErr};
use tp_client::contract::properties::states::IStates;

pub struct DeserializerBuilder<'a> {
    base: rs::Baseline,
    inst_contracts: InstantiatedContracts,
    data: &'a [u8],
    base_t: fb::Baseline<'a>,
    null_contract: NullContract,
    null_obj: rs::ObjectHandle,
}
impl<'a> DeserializerBuilder<'a> {
    pub fn new(data: &'a [u8], kind: rs::BaselineKind) -> Result<Self> {
        let base_t =
            flatbuffers::root::<fb::Baseline>(data).wrap_err("Error while verifying flatbuffer")?;

        let mut base = rs::Baseline::new(kind);
        let null_contract: NullContract = base.register_contract().unwrap();
        let null_obj = base
            .object_create(&null_contract, [].into_iter(), [].into_iter())
            .unwrap();

        Ok(Self {
            base,
            inst_contracts: InstantiatedContracts::new(),
            data,
            base_t,
            null_contract,
            null_obj,
        })
    }

    /// Call this once for each contract.
    pub fn register_contract<C: rs::Contract>(&mut self) -> Result<C> {
        // Yes this is not super efficient. But who cares, this is the simplest to understand.
        let idx = find_serialized_contract::<C>(self.base_t)
            .wrap_err("Failed to find matching contract")?;
        let contract = self
            .base
            .register_contract::<C>()
            .wrap_err("Contract already existed")?;
        let handle = contract.handle();
        self.inst_contracts.add_contract(idx, handle);
        Ok(contract)
    }

    pub fn finish(self) -> Deserializer<'a> {
        Deserializer::new(self)
    }
}

pub struct Deserializer<'a> {
    b: DeserializerBuilder<'a>,
    inst_states: InstantiatedStates,
    inst_objects: InstantiatedObjects,
}
impl<'a> Deserializer<'a> {
    pub fn new(builder: DeserializerBuilder<'a>) -> Self {
        Self {
            b: builder,
            inst_states: InstantiatedStates::new(),
            inst_objects: InstantiatedObjects::new(),
        }
    }

    /// Deserializes all objects belonging to contract `c`.
    ///
    /// # Panics
    /// Will panic if the contract `c` was not registered already.
    pub fn deserialize_objects<C: rs::Contract>(&mut self, c: &C) -> Result<()> {
        assert!(
            self.b.inst_contracts.is_registered_handle(c.handle()),
            "Contract was not already registered"
        );

        let Some(objects_t) = self.b.base_t.objects() else {
            // No objects at all, so we are done.
            return Ok(());
        };

        let contract_idx = self.b.inst_contracts.get_idx(c.handle());

        for (obj_idx, obj_t) in objects_with_contract_idx(contract_idx, objects_t) {
            validate_obj_matches_contract(obj_t, c, &self.b.inst_contracts, self.b.base_t)
                .wrap_err("Object did not conform to its contract")?;
        }

        // let instantiated_states = self
        //     .deserialize_states()
        //     .wrap_err("Error while deserializing states")?;
        // let instantiated_objects = self
        //     .deserialize_objects(&instantiated_states)
        //     .wrap_err("Error while deserializing objects")?;
        // self.fix_null_states(&instantiated_states, &instantiated_objects);

        Ok(())
    }

    pub fn finish(mut self) -> Result<rs::Baseline> {
        use rs::Contract;
        self.fix_null_states();

        // This should also remove the null object
        self.b
            .base
            .unregister_contract::<NullContract>(self.b.null_contract.handle())
            .wrap_err("Could not remove NullContract")?;

        Ok(self.b.base)
    }

    /// Deserializes all states into the baseline. `State<ObjectHandle>`s will point to the null
    /// object. Metadata about the states is stored in `InstantiatedStates`
    fn deserialize_states(&mut self) -> Result<InstantiatedStates> {
        todo!()
    }

    /// Takes all deserialized states that hold the null object's handle, and sets them to their
    /// correct target object based on what was originally in the flatbuffer.
    fn fix_null_states(&mut self) {
        todo!()
    }
}

/// Filter to just the objects in the flatbuffer which have `contract_idx`.
///
/// Does no other validation of the object.
fn objects_with_contract_idx<'a>(
    contract_idx: ContractsIdx,
    objects_t: flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<fb::Object<'a>>>,
) -> impl Iterator<Item = (ObjectsIdx, fb::Object<'a>)> {
    objects_t
        .into_iter()
        .enumerate()
        .filter(move |(_obj_idx, obj)| {
            let Some(c) = obj.contract() else {
                return false;
            };
            c.idx() as usize == contract_idx.0
        })
        .map(|(obj_idx, obj)| (ObjectsIdx(obj_idx), obj))
}

/// Validates that the serialized object `obj_t` matches its `contract`.
///
/// Return Err if they don't match.
fn validate_obj_matches_contract<'a, C: rs::Contract>(
    obj_t: fb::Object<'a>,
    contract: &C,
    inst_contracts: &InstantiatedContracts,
    baseline_t: fb::Baseline<'a>,
) -> Result<()> {
    // Validate contract field
    {
        let contract_idx_expected = inst_contracts.get_idx(contract.handle());
        let contract_idx_found: ContractsIdx = {
            let c: fb::ContractDataHandle = obj_t
                .contract()
                .ok_or_else(|| eyre!("Object was missing contract field"))?;
            ContractsIdx(usize::from(c.idx()))
        };
        if contract_idx_expected != contract_idx_found {
            return Err(eyre!("Object's contract field did not match `contract`"));
        }
    }

    // Validate number of states matches
    {
        let num_states_expected = C::States::enumerate_types().len();
        let num_states_found = obj_t.states().map_or(0, |x| x.len());
        if num_states_found != num_states_expected {
            return Err(eyre!(
                "number of states in serialized object did not match contract"
            ));
        }
    }

    // From here on out, we are just validating that all the states have the right type.
    let Some(obj_states_t) = obj_t.states() else {
        // The contract matches and there are no states, so we are done already.
        return Ok(());
    };

    // Get the list of states, we will need it in a moment when we index into it.
    let states_t = baseline_t
        .states()
        .ok_or(eyre!("Expected at least one state in the baseline!"))?;

    // Iterator of all the states belonging to the object, but we have converted from
    // handles to the actual `fb::State`.
    let obj_states_t = obj_states_t
        .into_iter()
        .map(|s: fb::StateHandle| StatesIdx(usize::try_from(s.idx()).unwrap()))
        .map(|s_idx: StatesIdx| states_t.get(s_idx.0));

    let zip_states_and_expected_types =
        std::iter::zip(obj_states_t, C::States::enumerate_types().into_iter());

    // Check that state types match contract
    for (i, (obj_state_t, expected_typ)) in zip_states_and_expected_types.enumerate() {
        if obj_state_t.p_type() != *expected_typ {
            return Err(eyre!(
                "state {i}'s type was {:?} but expected {:?}",
                obj_state_t.p_type().variant_name().unwrap(),
                *expected_typ,
            ));
        }
    }

    Ok(())
}

/// Check that the contract exists in the flatbuffer somewhere, and get its index.
///
/// Validates that the stored contract's StateIds match.
fn find_serialized_contract<C: rs::Contract>(baseline_t: fb::Baseline) -> Result<ContractsIdx> {
    let Some(contracts_t) = baseline_t.contracts() else {
        return Err(eyre!("There are no contracts to deserialize"));
    };

    // Deserialization would be faster if we searched for *all* contracts we
    // wanted to deserialize here, and not just an O(n) search for a single one.
    // But I'm punting this optimization until we know we need it.
    let (contract_idx, _contract_t) = contracts_t
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
            // Check that StateIds match
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
        .ok_or(eyre!("Coult not find a matching contract!"))?;
    Ok(ContractsIdx(contract_idx))
}
