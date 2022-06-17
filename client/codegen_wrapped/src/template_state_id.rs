use crate::{
    type_info::{PrimitiveType, TypeInfo, ValidOwnershipSemantics},
    ClassData,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct CDStateId {
    /// The mangled name of the inner type (`F32`, `ObjectHandle`)
    inner_mangled_name: String,
}
impl ClassData<CDStateId> {
    fn new(type_info: &PrimitiveType) -> Self {
        ClassData {
            namespace_super: "Contract.Properties".to_string(),
            namespace_sub: "States".to_string(),
            class_ident: format!("StateId_{}", type_info.mangled_name()),
            new_args: "".to_owned(),
            new_expr: None,
            drop_ident: Some(format!(
                "generated.__Internal.TpClientContractPropertiesStatesStateId{}Drop",
                type_info.mangled_name()
            )),
            additional_methods: Some(CDStateId {
                inner_mangled_name: type_info.mangled_name().to_owned(),
            }),
        }
    }

    pub fn generate_class_data() -> Vec<Self> {
        PrimitiveType::types()
            .iter()
            .map(ClassData::<CDStateId>::new)
            .collect()
    }
}
