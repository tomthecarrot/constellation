use crate::action::{Action, ActionKind, ActionResult, Collaction, CollactionResult};
use crate::Realm;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError};

type TryApplyResult = Result<CollactionResult, TryRecvError>;

type ApplyResult = Result<CollactionResult, RecvTimeoutError>;

pub type ActionSender = Sender<Collaction>;

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
pub struct Engine {
    realm: Realm,
    receiver: Receiver<Collaction>,
}
impl Engine {
    pub fn new(realm: Realm, queue_capacity: Option<usize>) -> (Self, ActionSender) {
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

    fn apply_collaction(&mut self, collaction: Collaction) -> CollactionResult {
        // Keep track of applied Actions
        let mut applied_actions: Vec<&dyn Action> = Vec::new();

        // Iterate through all Actions in this Collaction.
        for action in collaction.actions() {
            let action_ref = action.as_ref();
            let action_result = self.apply_action(action_ref);
            applied_actions.push(action_ref);

            // If Action failed
            if !action_result {
                // Reverse already applied Actions.
                self.reverse_actions(applied_actions);

                // Bail and reject the whole Collaction.
                return Err(collaction);
            }
        }

        // If all Actions succeeded, approve the Collaction.
        Ok(collaction)
    }

    fn apply_action(&mut self, action: &dyn Action) -> ActionResult {
        // Every Action is approved in this single-Guest version of the Platform.
        let is_approved = true;

        match action.kind() {
            ActionKind::StateAssert => {
                todo!("Get data from Action and compare it with the BaselineFork.");
            }
            ActionKind::StateWrite => {
                todo!("Get data from Action and apply it to the BaselineFork.");
            }
            _ => {
                eprintln!("[Engine] Cannot apply Action: type is not yet implemented.");
            }
        }

        is_approved
    }

    // TODO[SER-260]: should these `reverse` methods have a return value?
    fn reverse_action(&mut self, action: &dyn Action) {
        todo!("Reverse Action by applying the previous value to the BaselineFork.");
    }

    fn reverse_actions(&mut self, actions: Vec<&dyn Action>) {
        for action in actions {
            self.reverse_action(action);
        }
    }
}
