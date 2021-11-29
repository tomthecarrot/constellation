use crate::action::{Action, ActionKind, ActionResult, Collaction, CollactionResult};
use crate::baseline::BaselineKind;
use crate::contract::properties::TPData;
use crate::Realm;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError};

type TryApplyResult = Result<CollactionResult, TryRecvError>;

type ApplyResult = Result<CollactionResult, RecvTimeoutError>;

pub type ActionSender<T> = Sender<Collaction<T>>;

/// Manages reading and writing to the `Realm`.
///
/// # Threading architecture
/// The Engine has a queue of pending collactions that indend to mutate the
/// [`Realm`], as well as working copy of the `Realm` state. To avoid data races
/// the `Realm` is never simultaneously readable and writable at the same time.
///
/// The `Engine` cannot be simultaneously written and read from. For this
/// reason, typically things are done in two steps: a writer phase where
/// collactions are dequeued and applied as mutations on the `Realm` state, and
/// a reader phase where all reads of the data take place, free of any mutation.
/// Handling the transitions between these phases is the responsibility of the
/// API Client(s).
pub struct Engine<T: TPData> {
    realm: Realm,
    receiver: Receiver<Collaction<T>>,
}
impl<T: TPData + PartialEq + Clone> Engine<T> {
    pub fn new(realm: Realm, queue_capacity: Option<usize>) -> (Self, ActionSender<T>) {
        let (sender, receiver) = if let Some(cap) = queue_capacity {
            crossbeam_channel::bounded(cap)
        } else {
            crossbeam_channel::unbounded()
        };

        let this = Self { realm, receiver };
        (this, sender)
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
        let collaction = self.receiver.try_recv()?;
        let result = self.apply_collaction(collaction);
        Ok(result)
    }

    /// Blocks until a collaction is applied or rejected from the pending
    /// collactions, and returns the `CollactionResult`. If there are no
    /// collactions found by `timeout`, returns an error.
    pub fn apply_timeout(&mut self, timeout: std::time::Duration) -> ApplyResult {
        let collaction = self.receiver.recv_timeout(timeout)?;
        let result = self.apply_collaction(collaction);
        Ok(result)
    }

    fn apply_collaction(&mut self, collaction: Collaction<T>) -> CollactionResult {
        // Keep track of applied Actions
        let mut applied_actions: Vec<Box<dyn Action<T>>> = Vec::new();

        // Iterate through all Actions in this Collaction.
        for action in collaction.actions() {
            let action_result = self.apply_action(action);
            match action_result {
                Ok(action) => {
                    // Keep track of previously-applied Actions.
                    applied_actions.push(action);
                }
                Err(action) => {
                    // Reverse previously-applied Actions within this Collaction.
                    applied_actions.push(action);
                    self.reverse_actions(applied_actions);

                    // Bail and reject this Collaction.
                    return Err(false);
                }
            }
        }

        // If all Actions succeeded, approve the Collaction.
        Ok(true)
    }

    fn apply_action(&mut self, action: Box<dyn Action<T>>) -> ActionResult<T> {
        let mut is_approved = false;

        match action.kind() {
            ActionKind::StateAssert => {
                // Get data from the Action and compare it against the BaselineFork.
                let state_handle = action.state_handle();
                let data_new = action.raw_data();
                let state_result = self
                    .realm()
                    .baseline(BaselineKind::Fork)
                    .state(state_handle);

                match state_result {
                    Ok(state) => {
                        if &state.0 == data_new {
                            is_approved = true
                        }
                    }
                    Err(e) => {
                        panic!("[Engine] Could not apply StateAssert action: {}", e);
                    }
                }
            }
            ActionKind::StateWrite => {
                // Get data from the Action and apply it to the BaselineFork.
                let state_handle = action.state_handle();
                let data_new = action.raw_data();
                let state_result = self
                    .realm_mut()
                    .baseline_mut(BaselineKind::Fork)
                    .state_mut(state_handle);

                match state_result {
                    Ok(state) => {
                        state.0 = data_new.clone(); // TODO[SER-260]: this deep copy seems inefficient.
                    }
                    Err(e) => {
                        panic!("[Engine] Could not apply StateWrite action: {}", e);
                    }
                }
            }
            _ => {
                panic!(
                    "[Engine] Cannot apply Action of specified ActionKind: not yet implemented."
                );
            }
        }

        if is_approved {
            Ok(action)
        } else {
            Err(action)
        }
    }

    // TODO[SER-260]: should these `reverse` methods have a return value?
    fn reverse_action(&mut self, action: Box<dyn Action<T>>) {
        todo!("Reverse Action by applying the previous value to the BaselineFork.");
    }

    fn reverse_actions(&mut self, actions: Vec<Box<dyn Action<T>>>) {
        for action in actions {
            self.reverse_action(action);
        }
    }
}
