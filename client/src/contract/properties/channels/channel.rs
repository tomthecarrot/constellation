use crate::contract::properties::traits::ITpProperty;

// TODO: figure out data in a channel
pub struct Channel<T: ITpProperty>(pub T);
