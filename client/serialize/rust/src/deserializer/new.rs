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

use bimap::BiHashMap;
use std::collections::HashMap;

use crate::rs;
use crate::types::{ContractsIdx, ObjectsIdx, StatesIdx};

/// Tracks the relationships between indicies in the flatbuffer and instantiated
/// contracts in the baseline.
struct InstantiatedContracts(BiHashMap<ContractsIdx, rs::ContractDataHandle>);
impl InstantiatedContracts {
    /// None if the contract wasn't instantiated yet.
    fn get_handle(&self, idx: ContractsIdx) -> Option<rs::ContractDataHandle> {
        self.0.get_by_left(&idx).copied()
    }

    fn get_idx(&self, handle: rs::ContractDataHandle) -> ContractsIdx {
        self.0
            .get_by_right(&handle)
            .copied()
            .expect("Only instantiated contract handles should have been used. Howmst")
    }
}

/// Tracks the relationships between indicies in the flatbuffer and instantiated states
/// in the baseline. This assumes that all `State<ObjectHandle>`s in the baseline are
/// supposed to point to the null object at this stage in deserialization.
struct InstantiatedStates {
    /// All instantiated states, including ones that reference null objects.
    instantiations: BiHashMap<StatesIdx, rs::DynStateHandle>,
    /// A mapping of null states to the objects in the flatbuffer they are supposed
    /// to reference.
    null_map: HashMap<rs::StateHandle<rs::ObjectHandle>, ObjectsIdx>,
}
impl InstantiatedStates {
    /// `None` if the contract wasn't instantiated yet.
    fn get_handle(&self, idx: StatesIdx) -> Option<rs::DynStateHandle> {
        self.instantiations.get_by_left(&idx).copied()
    }

    fn get_idx(&self, handle: rs::DynStateHandle) -> StatesIdx {
        self.instantiations
            .get_by_right(&handle)
            .copied()
            .expect("Only instantiated state handles should have been used. Howmst?")
    }

    fn is_null_idx(&self, idx: StatesIdx) -> bool {
        let h = self
            .get_handle(idx)
            .expect("idx has no corresponding handle.");
        self.is_null_handle(h)
    }

    fn is_null_handle(&self, handle: rs::DynStateHandle) -> bool {
        use tp_client::contract::properties::states::dyn_handle::DynStateHandlePrimitive as P;
        use tp_client::contract::properties::states::DynStateHandle;
        let h = match handle {
            DynStateHandle::Primitive(p) => match p {
                P::ObjectHandle(h) => h,
                _ => return false,
            },
            DynStateHandle::Vec(_) => panic!("unsupported"),
        };
        assert!(
            self.null_map.contains_key(&h),
            "this is a bug: All object handles should be null"
        );
        return true;
    }

    fn get_original_idx(&self, handle: rs::StateHandle<rs::ObjectHandle>) -> ObjectsIdx {
        self.null_map
            .get(&handle)
            .copied()
            .expect("`handle` should have been present in the list of null states, but wasnt")
    }
}
