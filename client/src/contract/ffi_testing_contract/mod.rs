use crate::contract::{states, Contract, ContractDataHandle, ContractId};

pub struct FfiTestingContract {
    handle: ContractDataHandle,
    states: States,
    channels: (),
}

#[derive(Debug)]
#[states]
pub struct States {
    test_u8: u8,
    test_u16: u16,
    test_i64: i64,
}

impl Contract for FfiTestingContract {
    type States = States;
    type Channels = ();

    const ID: ContractId = ContractId {
        name: "teleportal.app-ffi_testing-FfiDefaultContract",
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
