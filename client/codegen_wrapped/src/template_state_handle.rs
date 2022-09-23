use crate::{
    type_info::{PrimitiveType, TypeInfo},
    ClassData,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct CDStateHandle;
impl ClassData<CDStateHandle> {
    fn new(type_info: &PrimitiveType) -> Self {
        ClassData {
            namespace_super: "Contract.Properties".to_string(),
            namespace_sub: "States".to_string(),
            class_ident: format!("StateHandle_{}", type_info.mangled_name()),
            only_owned: true,
            new_args: "".to_owned(),
            new_expr: None,
            drop_ident: Some(format!(
                "generated.__Internal.ConstellationContractPropertiesStatesStateHandle{}Drop",
                type_info.mangled_name()
            )),
            additional_methods: None,
        }
    }

    pub fn generate_class_data() -> Vec<Self> {
        PrimitiveType::types()
            .iter()
            .map(ClassData::<CDStateHandle>::new)
            .collect()
    }
}
