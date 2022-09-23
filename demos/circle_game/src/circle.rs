use constellation::contract::{states, Contract, ContractDataHandle, ContractId};

#[derive(Debug)]
pub struct Circle {
    handle: ContractDataHandle,
    states: States,
    channels: (),
}

#[derive(Debug)]
#[states]
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

    fn new(handle: ContractDataHandle) -> Self {
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

    fn handle(&self) -> ContractDataHandle {
        self.handle
    }
}
