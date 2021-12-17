use tp_client::contract::{states, Contract, ContractId};

pub struct Circle {
    states: States,
    channels: (),
}

#[states]
pub struct States {
    x: f32,
    y: f32,
}

impl Contract for Circle {
    type States = States;
    type Channels = ();

    const ID: ContractId = ContractId();
    const NAME: &'static str = "teleportal.app-circle_game-Circle";
    const VERSION: (u16, u16, u16) = (0, 0, 1);

    fn states(&self) -> &Self::States {
        &self.states
    }
    fn channels(&self) -> &Self::Channels {
        &self.channels
    }
}
