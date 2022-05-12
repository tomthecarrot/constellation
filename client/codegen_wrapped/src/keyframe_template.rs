use crate::{ClassData, TypeInfo, TYPES_INFO};

use serde::Serialize;

#[derive(Serialize)]
pub struct Kf {
    type_cs: String,
    type_platform: String,
}
impl ClassData<Kf> {
    fn new(type_info: &TypeInfo) -> Self {
        ClassData {
            class_ident: format!("Keyframe_{}", type_info.type_platform),
            new_args: format!("{} value, double time", type_info.type_cs),
            new_expr: if type_info.has_new {
                Some(format!("generated.__Internal.TpClientContractPropertiesChannelsKeyframe{0}New(RSharp.RBox_{0}.new_(value), time)", type_info.type_platform))
            } else {
                None
            },
            drop_ident: format!(
                "generated.__Internal.TpClientContractPropertiesChannelsKeyframe{}Drop",
                type_info.type_platform
            ),
            additional_methods: Some(Kf {
                type_cs: type_info.type_cs.to_string(),
                type_platform: type_info.type_platform.to_string(),
            }),
        }
    }

    pub fn generate_class_data() -> Vec<Self> {
        TYPES_INFO.iter().map(ClassData::<Kf>::new).collect()
    }
}
