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
use crate::types::ContractsIdx;
use crate::{fb, rs};

use eyre::{eyre, Result, WrapErr};
use tp_client::contract::properties::states::IStates;

pub struct Deserializer<'a> {
    b: rs::Baseline,
    contracts: InstantiatedContracts,
    data: &'a [u8],
    baseline_t: fb::Baseline<'a>,
    null_contract: NullContract,
    null_obj: rs::ObjectHandle,
}
impl<'a> Deserializer<'a> {
    pub fn new(data: &'a [u8], kind: rs::BaselineKind) -> Result<Self> {
        let baseline_t =
            flatbuffers::root::<fb::Baseline>(data).wrap_err("Error while verifying flatbuffer")?;

        let mut b = rs::Baseline::new(kind);
        let null_contract: NullContract = b.register_contract().unwrap();
        let null_obj = b
            .object_create(&null_contract, [].into_iter(), [].into_iter())
            .unwrap();

        Ok(Deserializer {
            b,
            contracts: InstantiatedContracts::new(),
            data,
            baseline_t,
            null_contract,
            null_obj,
        })
    }

    /// Call this once for each contract.
    pub fn register_contract<C: rs::Contract>(&mut self) -> Result<C> {
        // Yes this is not super efficient. But who cares, this is the simplest to understand.
        let idx = find_matching_contract::<C>(self.baseline_t)
            .wrap_err("Failed to find matching contract")?;
        let contract = self
            .b
            .register_contract::<C>()
            .wrap_err("Contract already existed")?;
        let handle = contract.handle();
        self.contracts.register_contract(idx, handle);
        Ok(contract)
    }

    /// Finishes serialization.
    pub fn finish(mut self) -> Result<rs::Baseline> {
        let instantiated_states = self
            .deserialize_states()
            .wrap_err("Error while deserializing states")?;
        let instantiated_objects = self
            .deserialize_objects(&instantiated_states)
            .wrap_err("Error while deserializing objects")?;
        self.fix_null_states(&instantiated_states, &instantiated_objects);

        use rs::Contract;
        self.b
            .unregister_contract::<NullContract>(self.null_contract.handle())
            .wrap_err("Could not remove NullContract")?;
        Ok(self.b)
    }

    /// Deserializes all states into the baseline. `State<ObjectHandle>`s will point to the null
    /// object. Metadata about the states is stored in `InstantiatedStates`
    fn deserialize_states(&mut self) -> Result<InstantiatedStates> {
        todo!()
    }

    /// Deserializes all objects into the baseline.
    fn deserialize_objects(&mut self, states: &InstantiatedStates) -> Result<InstantiatedObjects> {
        todo!()
    }

    /// Takes all deserialized states that hold the null object's handle, and sets them to their
    /// correct target object based on what was originally in the flatbuffer.
    fn fix_null_states(&mut self, states: &InstantiatedStates, objects: &InstantiatedObjects) {
        todo!()
    }
}

fn find_matching_contract<C: rs::Contract>(baseline_t: fb::Baseline) -> Result<ContractsIdx> {
    // Check that the contract exists in the flatbuffer somewhere, and get its index
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
        .ok_or(eyre!("Coult not find a matching contract!"))?;

    Ok(ContractsIdx(contract_idx))
}
