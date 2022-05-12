use crate::ClassData;

use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    // Platform Type | C# Type
    static ref TYPES_INFO: Vec<TypeInfo> = Vec::from([
        TypeInfo::new("U8", "byte", true),
        TypeInfo::new("U16", "ushort", true),
        TypeInfo::new("U32", "uint", true),
        TypeInfo::new("U64", "ulong", true),
        TypeInfo::new("I8", "sbyte", true),
        TypeInfo::new("I16", "short", true),
        TypeInfo::new("I32", "int", true),
        TypeInfo::new("I64", "long", true),
        TypeInfo::new("Bool", "bool", true),
        TypeInfo::new("F32", "float", true),
        TypeInfo::new("F64", "double", true),
        TypeInfo::new("ObjectHandle", "IntPtr", false),
        TypeInfo::new("ContractDataHandle", "IntPtr", false),
    ]);
}

struct TypeInfo {
    type_platform: &'static str,
    type_cs: &'static str,
    has_new: bool,
}
impl TypeInfo {
    pub fn new(type_platform: &'static str, type_cs: &'static str, has_new: bool) -> Self {
        Self {
            type_platform,
            type_cs,
            has_new,
        }
    }
}

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
