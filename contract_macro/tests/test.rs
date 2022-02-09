use std::any::TypeId;
use tp_contract_macro::{channels, states};

#[test]
fn test_macros() {
    use tp_client::contract::properties::channel::IChannels;
    use tp_client::contract::properties::state::IStates;

    #[states]
    pub struct MyStates {
        s1: u32,
        f1: f64,
    }

    #[channels]
    pub struct MyChannels {
        c1: f32,
        c_whatever: u8,
    }

    assert_eq!(
        MyStates::type_ids(),
        &[TypeId::of::<u32>(), TypeId::of::<f64>()],
    );

    assert_eq!(
        MyChannels::type_ids(),
        &[TypeId::of::<f32>(), TypeId::of::<u8>()],
    );
}
