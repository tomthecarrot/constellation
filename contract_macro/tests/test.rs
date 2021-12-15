use tp_contract_macro::{channels, states};

#[test]
fn test_macros() {
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
}
