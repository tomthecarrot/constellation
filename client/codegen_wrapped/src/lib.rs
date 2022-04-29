use handlebars::Handlebars;
use lazy_static::lazy_static;
use miette::{miette, IntoDiagnostic, Result, WrapErr};
use serde::Serialize;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const TPL_NAME: &'static str = "tpl";

#[derive(Clone, Serialize)]
pub struct ClassData {
    pub class_ident: String,
    pub new_args: String,
    pub new_expr: String,
    pub drop_ident: String,
    pub additional_methods: Option<String>,
}

pub trait ClassDataTemplate {
    fn template() -> ClassData;
}

lazy_static! {
    // Platform Type | C# Type | C# Pointer Type
    static ref TYPES: Vec<(&'static str, &'static str, &'static str)> = Vec::from([
        ("U8", "byte", "byte*"),
        ("U16", "ushort", "ushort*"),
        ("U32", "uint", "uint*"),
        ("U64", "ulong", "ulong*"),
        ("I8", "sbyte", "sbyte*"),
        ("I16", "short", "short*"),
        ("I32", "int", "int*"),
        ("I64", "long", "long*"),
        ("Bool", "bool", "bool*"),
        ("F32", "float", "float*"),
        ("F64", "double", "double*"),
        ("ObjectHandle", "IntPtr", "IntPtr"),
        ("ContractDataHandle", "IntPtr", "IntPtr"),
    ]);
}

pub struct Codegen {
    reg: Handlebars<'static>,
    tpl_path: PathBuf,
    output_dir: PathBuf,
}
impl Codegen {
    pub fn new() -> Result<Self> {
        let tpl_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("template.cs.tpl");
        let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("cs/src/generated/wrapped");

        if output_dir.exists() && output_dir.read_dir().into_diagnostic()?.next().is_some() {
            return Err(miette!(format!(
                "`output_dir` is not empty! Please delete it. (output_dir={output_dir:?})"
            )));
        }
        std::fs::create_dir_all(&output_dir)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to create `output_dir`={output_dir:?}"))?;

        let mut reg = handlebars::Handlebars::new();
        // https://docs.rs/handlebars/latest/handlebars/#strict-mode
        reg.set_strict_mode(true);
        // Don't escape characters
        reg.register_escape_fn(|s| s.to_string());

        reg.register_template_file(TPL_NAME, &tpl_path)
            .into_diagnostic()?;

        Ok(Self {
            reg,
            tpl_path,
            output_dir,
        })
    }

    pub fn substitute_and_write_class_data(&self, data: &ClassData) -> Result<()> {
        let class_ident = &data.class_ident;
        let output_path = self.output_dir.join(format!("{class_ident}.cs"));
        let output_file = File::create(&output_path)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("Failed to create output file for class {class_ident} at {output_path:?}")
            })?;

        self.reg
            .render_to_write(TPL_NAME, data, output_file)
            .into_diagnostic()
            .wrap_err("Failed to render to file")
    }

    pub fn monomorphize_templated_class_data(&self, templates: Vec<ClassData>) -> Vec<ClassData> {
        let mut output: Vec<ClassData> = Vec::new();
        for type_ in TYPES.iter() {
            for template in templates.iter() {
                let mut data = template.clone();
                data.class_ident = data
                    .class_ident
                    .replace("<type_platform>", type_.0)
                    .replace("<type_cs>", type_.1)
                    .replace("<type_cs_ptr>", type_.2);
                data.new_args = data
                    .new_args
                    .replace("<type_platform>", type_.0)
                    .replace("<type_cs>", type_.1)
                    .replace("<type_cs_ptr>", type_.2);
                data.new_expr = data
                    .new_expr
                    .replace("<type_platform>", type_.0)
                    .replace("<type_cs>", type_.1)
                    .replace("<type_cs_ptr>", type_.2);
                data.drop_ident = data
                    .drop_ident
                    .replace("<type_platform>", type_.0)
                    .replace("<type_cs>", type_.1)
                    .replace("<type_cs_ptr>", type_.2);

                if let Some(additional_methods) = data.additional_methods {
                    data.additional_methods = Some(
                        additional_methods
                            .replace("<type_platform>", type_.0)
                            .replace("<type_cs>", type_.1)
                            .replace("<type_cs_ptr>", type_.2),
                    );
                }

                output.push(data);
            }
        }

        output
    }
}
