pub struct State;

pub type StateID = arena::Index<State>;

/// Represents a particular state of a contract. For actual state data of a
/// specific object, see [`StateID`].
pub struct StateProperty {
    idx: usize,
}
