pub mod properties;

use crate::contract::properties::{ChannelID, StateID};

use std::collections::HashMap;

pub struct Contract {
    state_names: HashMap<String, StateID>,
    channel_names: HashMap<String, ChannelID>,
}

impl Contract {
    pub fn new() -> Self {
        todo!()
    }

    /// # Examples
    ///
    /// ```no_run
    /// # use tp_client::contract::Contract;
    /// # let c = Contract::new();
    /// let s1 = c.state("path/to/state1").unwrap();
    /// // use s1 on objects
    /// ```
    pub fn state(&self, name: &str) -> Option<StateID> {
        let n = self.state_names.get(name);
        n.copied()
    }
}

pub type ContractHandle = arena::Index<Contract>;
