use crate::action::property::{PropertyAction, StateAction};
use crate::action::{Action, ActionKind, ActionResult, Collaction, CollactionResult, IAction};
use crate::baseline::BaselineKind;
use crate::realm::Realm;
use crate::time::Ticks;

use better_borrow::BBorrow;
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError};
use eyre::{eyre, WrapErr};

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

    pub fn tick(&mut self, ticks_since_last_call: Ticks) {
        *self.realm.time_mut().ticks_mut() += ticks_since_last_call;
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

    fn apply_collaction(&mut self, mut collaction: Collaction) -> CollactionResult {
        // Keep track of applied Actions
        let mut applied_actions = Vec::new();

        // Iterate through all Actions in this Collaction.
        let actions = collaction.actions_mut();
        for action in actions {
            let action_result = self.apply_action(action);
            match action_result {
                Ok(()) => {
                    // Keep track of previously-applied Actions.
                    applied_actions.push(action);
                }
                Err(_) => {
                    // Reverse previously-applied Actions within this Collaction.
                    applied_actions.push(action);
                    // Go in FIFO order
                    self.reverse_actions(applied_actions.into_iter().rev());

                    // Bail and reject this Collaction.
                    return Err(collaction);
                }
            }
        }

        // If all Actions succeeded, approve the Collaction.
        Ok(collaction)
    }

    fn apply_action(&mut self, action: &mut Action) -> ActionResult {
        match action {
            Action::Property(PropertyAction::State(action)) => {
                // Get data from the Action and compare it against the BaselineFork.

                match action {
                    StateAction::Assert { handle, data } => {
                        let baseline = self.realm().baseline(BaselineKind::Fork);
                        let state = baseline.state(*handle).wrap_err("Invalid Handle")?;

                        if state.0 == BBorrow::borrow(data) {
                            Ok(())
                        } else {
                            Err(eyre!("Assert failed!"))
                        }
                    }
                    StateAction::Write { handle, data } => {
                        // Get data from the Action and apply it to the BaselineFork.
                        let state = self
                            .realm_mut()
                            .baseline_mut(BaselineKind::Fork)
                            .state_mut(*handle)
                            .wrap_err("Invalid Handle")?;

                        // Sanity check that all types remain the same
                        debug_assert!(state.0.prop_type() == data.prop_type());
                        // TODO[SER-272]: handle DynTpProperty<DynTpData> and DynTpData in general

                        // Swap the current value with the new data.
                        // This optimizes applying the Action and allows
                        // for its simple reversal if needed.

                        // TODO: hard ;(
                        //mem::swap(&mut state.value, data);
                        todo!("Figure out how to swap")
                    }
                }
            }
            _ => {
                tracing::warn!("Action not yet implemented. Treating as no-op.",);
                Ok(())
            }
        }
    }

    fn reverse_action(&mut self, action: &mut Action) {
        // Reverse Action by applying the previous value to the BaselineFork,
        // where applicable.
        match action.kind() {
            ActionKind::StateAssert => {} // no-op
            ActionKind::StateWrite => {
                // Reverse by re-applying the Action.
                // This triggers a value swap.
                self.apply_action(action)
                    .expect("TODO: How do we handle failure");
            }
            _ => {
                tracing::warn!(
                    "Reversing action that has not yet been implemented. Treating as no-op."
                );
            }
        }
    }

    fn reverse_actions<'a>(&'a mut self, actions: impl Iterator<Item = &'a mut Action>) {
        for action in actions {
            self.reverse_action(action);
        }
    }
}
