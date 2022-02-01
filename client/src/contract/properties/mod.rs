pub mod channel;
pub mod state;

// dyn_macro should not be used directly in the public API
#[doc(hidden)]
pub mod dyn_macro;

mod data;

pub use channel::{
    Channel, ChannelArenaHandle, ChannelArenaMap, ChannelHandle, ChannelId, ChannelsIter, IChannels,
};
pub use data::{DynTpData, DynTpProperty, ITpData, ITpProperty, TpDataType, TpPropertyType};
pub use state::{
    IStates, State, StateArenaHandle, StateArenaMap, StateHandle, StateId, StatesIter,
};

macro_rules! prop_iter {
    ($iter_name:ident, $trait_name:ident, $dyn_name:ident) => {
        pub struct $iter_name<S: $trait_name> {
            contract: ContractDataHandle,
            pos: usize,
            phantom: PhantomData<S>,
        }
        impl<S: $trait_name> $iter_name<S> {
            pub fn new(contract: ContractDataHandle) -> Self {
                Self {
                    contract,
                    pos: 0,
                    phantom: Default::default(),
                }
            }
        }

        impl<S: $trait_name> Iterator for $iter_name<S> {
            type Item = $dyn_name;

            fn next(&mut self) -> Option<Self::Item> {
                let prop_type = S::enumerate_types().get(self.pos).copied()?;
                let result = $dyn_name::new(self.contract, self.pos, prop_type);
                self.pos += 1;
                Some(result)
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let n_remaining = S::enumerate_types().len() - self.pos;
                (n_remaining, Some(n_remaining))
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let n_fields = S::enumerate_types().len();
                let new_pos = self.pos + n;
                if new_pos >= n_fields {
                    self.pos = n_fields;
                    return None;
                }
                let prop_type = S::enumerate_types()
                    .get(self.pos)
                    .copied()
                    .expect("Should be impossible to be `None`");
                self.pos += 1; // also discard the element at the new position
                Some($dyn_name::new(self.contract, self.pos, prop_type))
            }
        }
    };
}

use prop_iter; // re-export for use
