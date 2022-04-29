use crate::ClassData;
use lazy_static::lazy_static;

lazy_static! {
    // Platform Type | C# Type
    static ref TYPES: Vec<(&'static str, &'static str)> = Vec::from([
        ("U8", "byte"),
        ("U16", "ushort"),
        ("U32", "uint"),
        ("U64", "ulong"),
        ("I8", "sbyte"),
        ("I16", "short"),
        ("I32", "int"),
        ("I64", "long"),
        ("Bool", "bool"),
        ("F32", "float"),
        ("F64", "double"),
        ("ObjectHandle", "IntPtr"),
        ("ContractDataHandle", "IntPtr"),
    ]);
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
            additional_methods: Some(r#"
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
            "#.to_string()),
        }
    }

    pub fn generate_class_data(&self) -> Vec<ClassData> {
        let mut output: Vec<ClassData> = Vec::new();

        for type_ in TYPES.iter() {
            let class_ident = self
                .class_ident
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            let new_args = self
                .new_args
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            let new_expr = self
                .new_expr
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            let drop_ident = self
                .drop_ident
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            let additional_methods = self.additional_methods.as_ref().map(|value| {
                value
                    .replace("<type_platform>", type_.0)
                    .replace("<type_cs>", type_.1)
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
