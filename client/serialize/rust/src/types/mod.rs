mod eq_impl;
mod from_impl;

/// Index into `objects` vec
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ObjectsIdx(pub usize);

/// Index into `contracts` vec
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct ContractsIdx(pub usize);

/// Index into `states` vec
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct StatesIdx(pub usize);
