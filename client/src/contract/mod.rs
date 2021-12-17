pub mod properties;

pub use tp_contract_macro::{channels, states};

pub trait Contract {
    type States;
    type Channels;

    const ID: ContractId;
    const NAME: &'static str;
    const VERSION: (u16, u16, u16);

    fn states(&self) -> &Self::States;
    fn channels(&self) -> &Self::Channels;
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ContractId();
