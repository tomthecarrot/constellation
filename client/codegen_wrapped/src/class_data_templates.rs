use crate::ClassData;
use indoc::indoc;
use lazy_static::lazy_static;

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

pub struct KeyframeTemplate {
    class_ident: String,
    new_args: String,
    new_expr: String,
    drop_ident: String,
    additional_methods: Option<String>,
}

impl KeyframeTemplate {
    pub fn new() -> Self {
        Self {
            class_ident: "Keyframe_<type_platform>".to_string(),
            new_args: "<type_cs> value, double time".to_string(),
            new_expr: "generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>New(RSharp.RBox_<type_platform>.new_(value), time)".to_string(),
            drop_ident: "generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Drop".to_string(),
            additional_methods: Some(indoc! {r#"
                public unsafe <type_cs> Value
                {
                    get
                    {
                        var result = generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Value(this.Ptr?.p ?? IntPtr.Zero);
                        return ToManaged.f(OwnershipSemantics.SharedRef, result);
                    }
                }

                public double Time
                {
                    get => generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Time(this.Ptr?.p ?? IntPtr.Zero);
                }
            "#}.to_string()),
        }
    }

    pub fn generate_class_data(&self) -> Vec<ClassData> {
        let mut output: Vec<ClassData> = Vec::new();

        for type_info in TYPES_INFO.iter() {
            let class_ident = self
                .class_ident
                .replace("<type_platform>", type_info.type_platform)
                .replace("<type_cs>", type_info.type_cs);
            let new_args = self
                .new_args
                .replace("<type_platform>", type_info.type_platform)
                .replace("<type_cs>", type_info.type_cs);
            let new_expr = if type_info.has_new {
                Some(
                    self.new_expr
                        .replace("<type_platform>", type_info.type_platform)
                        .replace("<type_cs>", type_info.type_cs),
                )
            } else {
                None
            };
            let drop_ident = self
                .drop_ident
                .replace("<type_platform>", type_info.type_platform)
                .replace("<type_cs>", type_info.type_cs);
            let additional_methods = self.additional_methods.as_ref().map(|value| {
                value
                    .replace("<type_platform>", type_info.type_platform)
                    .replace("<type_cs>", type_info.type_cs)
            });

            output.push(ClassData {
                class_ident,
                new_args,
                new_expr,
                drop_ident,
                additional_methods,
            });
        }

        output
    }
}
