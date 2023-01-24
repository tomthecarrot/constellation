use tp_client::contract::{Contract, ContractDataHandle, ContractId};

pub struct NullContract {
    handle: ContractDataHandle,
}
impl Contract for NullContract {
    type States = ();
    type Channels = ();

    const ID: ContractId = ContractId {
        name: "teleportal.serialize.NullContract",
        version: (0, 0, 0),
    };

    fn new(handle: ContractDataHandle) -> Self {
        Self { handle }
    }

    fn states(&self) -> &Self::States {
        &()
    }

    fn channels(&self) -> &Self::Channels {
        &()
    }

    fn handle(&self) -> ContractDataHandle {
        self.handle
    }
}
