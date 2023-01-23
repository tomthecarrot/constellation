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
mod states;

use self::contracts::InstantiatedContracts;
use self::states::InstantiatedStates;
use crate::rs;
use crate::types::ContractsIdx;

pub struct Deserializer {
    //
}

pub struct DeserializerBuilder<'a> {
    b: rs::Baseline,
    contracts: InstantiatedContracts,
    bytes: &'a [u8],
}
impl<'a> DeserializerBuilder<'a> {
    pub fn new(bytes: &'a [u8], kind: rs::BaselineKind) -> Self {
        Self {
            b: rs::Baseline::new(kind),
            contracts: InstantiatedContracts::new(),
            bytes,
        }
    }
    pub fn register_contract<C: rs::Contract>(&mut self) -> C {
        // Check that the contract exists in the flatbuffer somewhere, and get its index
        let idx: ContractsIdx = { todo!() };
        let c = self
            .b
            .register_contract::<C>()
            .expect("Contract already existed");
        let handle = c.handle();
        self.contracts.register_contract(idx, handle);
        c
    }
}
