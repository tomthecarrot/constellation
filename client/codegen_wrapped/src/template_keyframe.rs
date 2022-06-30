use crate::{
    type_info::{PrimitiveType, TypeInfo, ValidOwnershipSemantics},
    ClassData,
};

use serde::Serialize;

#[derive(Serialize)]
pub struct CDKeyframe {
    /// The name of the owned wrapper type in C#
    value_owned_ident: String,
    /// The mangled name of values stored in the Keyframe in C# (`F32`, `ObjectHandle`)
    value_mangled_name: String,
    /// The T in Ptr<T> for the values
    value_ptr_inner: String,
    /// C# constructor has second arg
    has_second_arg: bool,
}
impl ClassData<CDKeyframe> {
    fn new(type_info: &PrimitiveType) -> Self {
        ClassData {
            namespace_super: "Contract.Properties".to_string(),
            namespace_sub: "Channels".to_string(),
            class_ident: format!("Keyframe_{}", type_info.mangled_name()),
            new_args: format!("{} value, double time", type_info.owned_ident()),
            new_expr: Some(format!(
                "generated.__Internal.TpClientContractPropertiesChannelsKeyframe{}New(
                    ({}) value.StealInner().p, time
                )",
                type_info.mangled_name(),
                type_info.ptr_raw(),
            )),
            drop_ident: Some(format!(
                "generated.__Internal.TpClientContractPropertiesChannelsKeyframe{}Drop",
                type_info.mangled_name()
            )),
            additional_methods: Some(CDKeyframe {
                value_owned_ident: type_info.owned_ident().to_owned(),
                value_mangled_name: type_info.mangled_name().to_owned(),
                value_ptr_inner: type_info.ptr_inner().to_owned(),
                has_second_arg: !matches!(
                    type_info.valid_ownership_semantics(),
                    ValidOwnershipSemantics::Owned
                ),
            }),
        }
    }

    pub fn generate_class_data() -> Vec<Self> {
        PrimitiveType::types()
            .iter()
            .map(ClassData::<CDKeyframe>::new)
            .collect()
    }
}
