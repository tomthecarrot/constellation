pub struct Channel {}

pub type ChannelID = arena::Index<Channel>;

/// Represents a particular channel of a contract. For actual channel data of a
/// specific object, see [`ChannelID`].
pub struct ChannelProperty {
    idx: usize,
}
