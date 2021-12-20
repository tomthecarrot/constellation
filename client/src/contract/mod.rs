use crate::realm::Realm;

pub mod properties;

pub use tp_contract_macro::{channels, states};

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

/// `ContractIdHandle`s are used to uniquely identify a particular contract within
/// the context of a particular engine. They are less expensive to compare than the
/// [`ContractId`]s they reference, so they are used instead most places. You need
/// to register a [`Contract`] with a [`Realm`] to get a `ContractIdHandle`.
pub type ContractIdHandle = arena::Index<ContractId>;

/// Contracts describe the valid set of properties in a category of objects, much
/// like a struct definition describes the variables in a particular instance of a
/// struct, or a class describes objects that are instances of that class.
///
/// Note that `Contract`s are not held internally by the [`Realm`], rather only
/// [`ContractId`]s and [`ContractIdHandle`]s. The API client should therefore hold
/// onto the instantiated `Contract` so that they can access the fields of any
/// particular object.
pub trait Contract {
    type States;
    type Channels;

    const ID: ContractId;

    fn new(handle: ContractIdHandle) -> Self;

    fn states(&self) -> &Self::States;
    fn channels(&self) -> &Self::Channels;
    fn handle(&self) -> ContractIdHandle;
}
