use tp_client::contract::{states, Contract, ContractId, ContractIdHandle};

#[derive(Debug)]
pub struct Circle {
    handle: ContractIdHandle,
    states: States,
    channels: (),
}

#[states]
#[derive(Debug)]
pub struct States {
    x: f32,
    y: f32,
}

impl Contract for Circle {
    type States = States;
    type Channels = ();

    const ID: ContractId = ContractId {
        name: "teleportal.app-circle_game-Circle",
        version: (0, 0, 1),
    };

    fn new(handle: ContractIdHandle) -> Self {
        Self {
            handle,
            states: States::new(handle),
            channels: Default::default(),
        }
    }

    fn states(&self) -> &Self::States {
        &self.states
    }

    fn channels(&self) -> &Self::Channels {
        &self.channels
    }

    fn handle(&self) -> ContractIdHandle {
        self.handle
    }
}
