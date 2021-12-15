pub mod properties;

pub trait Contract {
    type States;
    type Channels;

    fn states(&self) -> &Self::States;
    fn channels(&self) -> &Self::Channels;
    fn id(&self) -> ContractId;
    fn name(&self) -> &'static str;
    fn version(&self) -> &(u16, u16, u16);
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ContractId();
