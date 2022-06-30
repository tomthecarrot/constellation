use crate::{
    type_info::{PrimitiveType, TypeInfo},
    ClassData,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct CDState {
    /// The type of values stored in the State in C#
    type_cs: String,
    /// The type of values stored in the State in rust
    type_platform: String,
}
impl ClassData<CDState> {
    fn new(type_info: &PrimitiveType) -> Self {
        ClassData {
            namespace_super: "Contract.Properties".to_string(),
            namespace_sub: "States".to_string(),
            class_ident: format!("State_{}", type_info.mangled_name()),
            new_args: format!("{} value", type_info.owned_ident()),
            new_expr: if type_info.supports_new {
                Some(format!(
                    "generated.
                    __Internal.TpClientContractPropertiesStatesState{0}New(
                    new RSharp.RBox_{0}(value).StealInner()
                    )",
                    type_info.type_platform,
                ))
            } else {
                None
            },
            drop_ident: if type_info.supports_new {
                Some(format!(
                    "generated.__Internal.TpClientContractPropertiesStatesState{}Drop",
                    type_info.type_platform
                ))
            } else {
                None
            },
            ptr_literal: type_info.ptr_literal(),
            additional_methods: Some(CDState {
                type_cs: type_info.type_cs.to_string(),
                type_platform: type_info.type_platform.to_string(),
            }),
        }
    }

    pub fn generate_class_data() -> Vec<Self> {
        TYPES_INFO.iter().map(ClassData::<CDState>::new).collect()
    }
}
