pub use tp_contract_macro::{channels, states};
pub mod properties;

use crate::contract::properties::{IChannels, IStates};
use crate::object::ObjectHandle;

use std::collections::HashSet;

/// Represents information that globally and uniquely identifies a contract.
/// Any two contracts are the same if their `ContractId`s are equal.
///
/// Due to the cost of comparing two `ContractId`s, [`ContractIdHandle`]s are
/// typically used instead.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct ContractId {
    pub name: &'static str,
    pub version: (u16, u16, u16),
}

pub type ContractDataHandle = arena::Index<ContractData>;

/// Contracts describe the valid set of properties in a category of objects, much
/// like a struct definition describes the variables in a particular instance of a
/// struct, or a class describes objects that are instances of that class.
///
/// Note that `Contract`s are not held internally by the [`Realm`], rather only
/// [`ContractData`]s and [`ContractDataHandle`]s. The API client should therefore hold
/// onto the instantiated `Contract` so that they can access the fields of any
/// particular object.
pub trait Contract {
    type States: IStates;
    type Channels: IChannels;

    const ID: ContractId;

    fn new(handle: ContractDataHandle) -> Self;

    fn states(&self) -> &Self::States;
    fn channels(&self) -> &Self::Channels;
    fn handle(&self) -> ContractDataHandle;
}

/// Contains stateful data about the contract
pub struct ContractData {
    id: ContractId,
    objects: HashSet<ObjectHandle>,
}
impl ContractData {
    pub fn new(id: ContractId) -> Self {
        Self {
            id,
            objects: Default::default(),
        }
    }
    pub fn id(&self) -> ContractId {
        self.id
    }

    pub(super) fn objects_mut(&mut self) -> &mut HashSet<ObjectHandle> {
        &mut self.objects
    }

    pub fn objects(&self) -> &HashSet<ObjectHandle> {
        &self.objects
    }
}
