use serde::Serialize;
use std::collections::HashMap;

fn main() {
    let data_vec = generate_class_data();
    let serialized = serde_json::to_string(&data_vec).unwrap();
    println!("serialized: {0}", serialized);
}

fn generate_class_data() -> Vec<ClassData> {
    let types = HashMap::from([
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

    let mut templates: Vec<ClassData> = Vec::new();
    let template_keyframe = ClassData {
        class_ident: "Keyframe_<type_platform>".to_string(),
        new_args: "<type_cs> value, double time".to_string(),
        new_expr: "generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>New(RSharp.RBox_<type_platform>.new_(value), time)".to_string(),
        drop_ident: "generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Drop".to_string(),
        additional_methods: None,
    };
    templates.push(template_keyframe);

    let mut output: Vec<ClassData> = Vec::new();
    for type_ in types {
        for template in templates.iter() {
            let mut data = template.clone();
            data.class_ident = data
                .class_ident
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            data.new_args = data
                .new_args
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            data.new_expr = data
                .new_expr
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);
            data.drop_ident = data
                .drop_ident
                .replace("<type_platform>", type_.0)
                .replace("<type_cs>", type_.1);

            if let Some(additional_methods) = data.additional_methods {
                data.additional_methods = Some(
                    additional_methods
                        .replace("<type_platform>", type_.0)
                        .replace("<type_cs>", type_.1),
                );
            }

            output.push(data);
        }
    }

    output
}

#[derive(Clone, Serialize)]
struct ClassData {
    pub class_ident: String,
    pub new_args: String,
    pub new_expr: String,
    pub drop_ident: String,
    pub additional_methods: Option<String>,
}
