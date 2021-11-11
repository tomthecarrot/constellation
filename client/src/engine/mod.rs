use crate::Realm;

/// Manages reading and writing to the `Realm`.
///
/// # Threading architecture
/// The Engine has a queue of pending collactions that indend to mutate the
/// [`Realm`], as well as working copy of the `Realm` state. To avoid data races
/// the `Realm` is never simultaneously readable and writable at the same time.
///
/// The `Engine` operates in two steps: a writer phase where collactions are
/// dequeued and applied as mutations on the `Realm` state, and a reader phase
/// where all reads of the data take place, free of any mutation.
pub struct Engine {
    working_state: Realm,
}

pub struct ReadableEngine {
    realm: Realm,
}
