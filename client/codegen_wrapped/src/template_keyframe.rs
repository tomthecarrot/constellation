use crate::{ClassData, TypeInfo, TYPES_INFO};

use serde::Serialize;

#[derive(Serialize)]
pub struct CDKeyframe {
    type_cs: String,
    type_platform: String,
}
impl ClassData<CDKeyframe> {
    fn new(type_info: &TypeInfo) -> Self {
        ClassData {
            namespace_super: "Contract.Properties".to_string(),
            namespace_sub: "Channels".to_string(),
            class_ident: format!("Keyframe_{}", type_info.type_platform),
            new_args: format!("{} value, double time", type_info.type_cs),
            new_expr: if type_info.supports_new {
                Some(format!("generated.__Internal.TpClientContractPropertiesChannelsKeyframe{0}New(RSharp.RBox_{0}.new_(value), time)", type_info.type_platform))
            } else {
                None
            },
            drop_ident: Some(format!(
                "generated.__Internal.TpClientContractPropertiesChannelsKeyframe{}Drop",
                type_info.type_platform
            )),
            is_ptr_type: type_info.is_ptr_type,
            additional_methods: Some(CDKeyframe {
                type_cs: type_info.type_cs.to_string(),
                type_platform: type_info.type_platform.to_string(),
            }),
        }
    }

    pub fn generate_class_data() -> Vec<Self> {
        TYPES_INFO
            .iter()
            .map(ClassData::<CDKeyframe>::new)
            .collect()
    }
}
