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
use tp_client::contract::properties::dynamic::{DynTpPrimitive, DynTpProperty};
use tp_client::contract::properties::states::id::DynStateIdPrimitive;
use tp_client::contract::properties::states::{DynStateId, IStates};

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
    /// Will panic if the `contract` was not registered already.
    pub fn deserialize_objects<C: rs::Contract>(&mut self, contract: &C) -> Result<()> {
        assert!(
            self.b
                .inst_contracts
                .is_registered_handle(contract.handle()),
            "Contract was not already registered"
        );

        let Some(objects_t) = self.b.base_t.objects() else {
            // No objects at all, so we are done.
            return Ok(());
        };

        let contract_idx = self.b.inst_contracts.get_idx(contract.handle());

        for (obj_idx, _obj_t) in objects_with_contract_idx(contract_idx, objects_t) {
            let obj_valid: ValidatedObject = validate_obj_matches_contract(
                obj_idx,
                contract,
                &self.b.inst_contracts,
                self.b.base_t,
            )
            .wrap_err("Object did not conform to its contract")?;
            self.deserialize_obj_with_null::<C>(obj_valid, contract)
                .wrap_err("Failed to deserialize object")?;
        }

        Ok(())
    }

    pub fn finish(mut self) -> Result<rs::Baseline> {
        use rs::Contract;
        // Takes all deserialized states that hold the null object's handle, and sets them to their
        // correct target object based on what was originally in the flatbuffer.
        for (s_idx, s_handle, o_idx) in self.inst_states.iter() {
            let o_handle: rs::ObjectHandle = self.inst_objects.get_handle(o_idx).wrap_err(
                "A state referenced an object that didn't exist in the baseline. \
                Were all contracts' objects deserialized already?",
            )?;
            let state_ref: &mut rs::State<_> = self.b.base.state_mut(s_handle).wrap_err(
                "Null state was unexpectedly absent from the deserialized baseline. This is a bug.",
            )?;
            state_ref.value = o_handle;
        }

        // This should also remove the null object
        self.b
            .base
            .unregister_contract::<NullContract>(self.b.null_contract.handle())
            .wrap_err("Could not remove NullContract")?;

        Ok(self.b.base)
    }
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

/// An object in the flatbuffer that has already been validated to match its contract.
struct ValidatedObject<'a> {
    idx: ObjectsIdx,
    t: fb::Object<'a>,
}

/// Validates that the serialized object `obj_t` matches its `contract`.
///
/// Return Err if they don't match.
fn validate_obj_matches_contract<'a, C: rs::Contract>(
    obj_idx: ObjectsIdx,
    contract: &C,
    inst_contracts: &InstantiatedContracts,
    baseline_t: fb::Baseline<'a>,
) -> Result<ValidatedObject<'a>> {
    let obj_t: fb::Object = baseline_t
        .objects()
        .expect("Tried to validate an `ObjectsIdx` for a `fb::Baseline` without any objects!")
        .get(obj_idx.0);

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
        return Ok(ValidatedObject{idx: obj_idx, t: obj_t});
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

    Ok(ValidatedObject {
        idx: obj_idx,
        t: obj_t,
    })
}

impl<'a> Deserializer<'a> {
    /// Deserializes `obj` into the baseline, but any `State<ObjectHandle`s are set to
    /// the null object handle.
    fn deserialize_obj_with_null<C: rs::Contract>(
        &mut self,
        obj: ValidatedObject,
        contract: &C,
    ) -> Result<()> {
        // TODO: This could be an array if we had a const for `C`'s number of states.
        let mut obj_states: Vec<StatesIdx> = Vec::new();
        if let Some(obj_states_t) = obj.t.states() {
            assert_eq!(
                obj_states_t.len(),
                C::States::enumerate_types().len(),
                "sanity check"
            );

            obj_states.extend(
                obj_states_t
                    .into_iter()
                    .map(|h: fb::StateHandle| StatesIdx(usize::try_from(h.idx()).unwrap())),
            );
        }

        macro_rules! helper {
            ($e:expr) => {{
                DynTpProperty::Primitive(DynTpPrimitive::from($e.to_owned()))
            }};
        }

        let mut dyn_props: Vec<DynTpProperty> = Vec::new();
        // Used to track which states are null states temporarily. We don't have the
        // `rs::StateHandle` for the state until after we construct the object, so this
        // will be used after object construction to re-associate these `StatesIdx`
        // with the `rs::StateHandle`.
        let mut null_states: Vec<(rs::StateId<rs::ObjectHandle>, StatesIdx)> = Vec::new();
        for (state_id, obj_state_idx) in contract.state_iter().zip(obj_states.into_iter()) {
            let obj_state_t = self.b.base_t.states().unwrap().get(obj_state_idx.0);
            // Handle dynamic typing of union to access the property
            use fb::TpPrimitive as P;

            let prop = match obj_state_t.p_type() {
                P::U8 => helper!(obj_state_t.p_as_u8().unwrap().v()),
                P::U16 => helper!(obj_state_t.p_as_u16().unwrap().v()),
                P::U32 => helper!(obj_state_t.p_as_u32().unwrap().v()),
                P::U64 => helper!(obj_state_t.p_as_u64().unwrap().v()),
                P::I8 => helper!(obj_state_t.p_as_i8().unwrap().v()),
                P::I16 => helper!(obj_state_t.p_as_i16().unwrap().v()),
                P::I32 => helper!(obj_state_t.p_as_i32().unwrap().v()),
                P::I64 => helper!(obj_state_t.p_as_i64().unwrap().v()),
                P::Bool => helper!(obj_state_t.p_as_bool().unwrap().v()),
                P::F32 => helper!(obj_state_t.p_as_f32().unwrap().v()),
                P::F64 => helper!(obj_state_t.p_as_f64().unwrap().v()),
                P::FbString => helper!(obj_state_t.p_as_fb_string().unwrap().v().unwrap()),
                P::tp_serialize_object_ObjectHandle => {
                    // Figure out what object was referenced in the state, and track it.
                    let referenced_obj_handle_t: fb::ObjectHandle = obj_state_t
                        .p_as_tp_serialize_object_object_handle()
                        .unwrap();
                    let referenced_obj_idx =
                        ObjectsIdx(usize::try_from(referenced_obj_handle_t.idx()).unwrap());
                    self.inst_states
                        .track_obj_reference(obj_state_idx, referenced_obj_idx)?;

                    let DynStateId::Primitive(DynStateIdPrimitive::ObjectHandle(state_id)) = state_id else {
                        unreachable!("We already validated that the state type should match the contract");
                    };
                    // Mark our state as a null state.
                    null_states.push((state_id, obj_state_idx));

                    // Set to the null object
                    DynTpProperty::Primitive(DynTpPrimitive::ObjectHandle(self.b.null_obj))
                }
                P::tp_serialize_contract_ContractDataHandle => {
                    let contract_handle_t: fb::ContractDataHandle = obj_state_t
                        .p_as_tp_serialize_contract_contract_data_handle()
                        .unwrap();
                    let contract_idx =
                        ContractsIdx(usize::try_from(contract_handle_t.idx()).unwrap());
                    let contract_handle: rs::ContractDataHandle = self
                        .b
                        .inst_contracts
                        .get_handle(contract_idx)
                        .ok_or_else(|| eyre!("Contract was missing from registry"))?;

                    DynTpProperty::Primitive(DynTpPrimitive::ContractDataHandle(contract_handle))
                }
                _ => unimplemented!("Other types are not supported."),
            };
            dyn_props.push(prop);
        }

        let new_obj_handle: rs::ObjectHandle = self
            .b
            .base
            .object_create(contract, dyn_props.into_iter(), [].into_iter())
            .wrap_err("failed to create object")?;

        // Go back through all marked null states and actually associate their idx with
        // their handle.
        for (null_state_id, null_state_idx) in null_states {
            let null_state_handle: rs::StateHandle<rs::ObjectHandle> = self
                .b
                .base
                .bind_state(null_state_id, new_obj_handle)
                .expect("impossible: we already serialized the state");
            self.inst_states
                .track_instantiated_state(null_state_idx, null_state_handle)?;
        }

        self.inst_objects.add_object(new_obj_handle, obj.idx);

        Ok(())
    }
}
