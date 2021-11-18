//! Manages reading and writing to the `Realm`.
//!
//! # Threading architecture
//! The Engine has a queue of pending collactions that indend to mutate the
//! [`Realm`], as well as working copy of the `Realm` state. To avoid data races
//! the `Realm` is never simultaneously readable and writable at the same time.
//!
//! The `Engine` operates in two steps: a writer phase where collactions are
//! dequeued and applied as mutations on the `Realm` state, and a reader phase
//! where all reads of the data take place, free of any mutation.

use crate::action::{Collaction, CollactionResult};
use crate::Realm;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError};

type TryApplyResult = Result<CollactionResult, TryRecvError>;

type ApplyResult = Result<CollactionResult, RecvTimeoutError>;

/// The writeable state of the engine. In this state, the realm's data can be
/// mutated by a single thread.
pub struct WriteableEngine {
    realm: Realm,
    receiver: Receiver<Collaction>,
}
impl WriteableEngine {
    pub fn new(realm: Realm, queue_capacity: Option<usize>) -> (Self, Sender<Collaction>) {
        let (sender, receiver) = if let Some(cap) = queue_capacity {
            crossbeam_channel::bounded(cap)
        } else {
            crossbeam_channel::unbounded()
        };

        let this = Self { realm, receiver };
        (this, sender)
    }
    pub fn into_readable(self) -> ReadableEngine {
        ReadableEngine {
            realm: self.realm,
            receiver: self.receiver,
        }
    }

    pub fn realm(&self) -> &Realm {
        &self.realm
    }

    pub fn realm_mut(&mut self) -> &mut Realm {
        &mut self.realm
    }

    /// Same as `apply_timeout()`, but immediately returns if there are no
    /// collactions pending.
    pub fn try_apply(&mut self) -> TryApplyResult {
        let c = self.receiver.try_recv()?;
        todo!("apply collaction!")
    }

    /// Blocks until a collaction is applied or rejected from the pending
    /// collactions, and returns the `CollactionResult`. If there are no
    /// collactions found by `timeout`, returns an error.
    pub fn apply_timeout(&mut self, timeout: std::time::Duration) -> ApplyResult {
        let c = self.receiver.recv_timeout(timeout)?;
        todo!("apply collaction!")
    }
}

/// The readable state of the engine. In this state, the realm's data can be
/// concurrently read by multiple threads, but will not be mutated.
pub struct ReadableEngine {
    realm: Realm,
    receiver: Receiver<Collaction>,
}
impl ReadableEngine {
    pub fn realm(&self) -> &Realm {
        &self.realm
    }

    pub fn into_writeable(self) -> WriteableEngine {
        WriteableEngine {
            realm: self.realm,
            receiver: self.receiver,
        }
    }
}
