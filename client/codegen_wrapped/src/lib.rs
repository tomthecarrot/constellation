mod keyframe_template;
mod state_template;

pub use keyframe_template::KeyframeTemplate;
pub use state_template::StateTemplate;

use handlebars::Handlebars;
use lazy_static::lazy_static;
use miette::{miette, IntoDiagnostic, Result, WrapErr};
use serde::Serialize;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const TPL_NAME: &'static str = "tpl";

lazy_static! {
    // Platform Type | C# Type
    pub static ref TYPES_INFO: Vec<TypeInfo> = Vec::from([
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

pub struct TypeInfo {
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

#[derive(Clone, Serialize)]
pub struct ClassData {
    pub namespace_super: String,
    pub namespace_sub: String,
    pub class_ident: String,
    pub new_args: String,
    pub new_expr: Option<String>,
    pub drop_ident: String,
    pub additional_methods: Option<String>,
}

pub trait ClassDataTemplate {
    fn namespace_super() -> String;
    fn namespace_sub() -> String;
    fn class_ident() -> String;
    fn new_args() -> String;
    fn new_expr() -> Option<String>;
    fn drop_ident() -> String;
    fn additional_methods() -> Option<String>;
    fn generate_class_data() -> Vec<ClassData>;
}

pub fn generate_class_data_generic<T: ClassDataTemplate>() -> Vec<ClassData> {
    let mut output: Vec<ClassData> = Vec::new();

    for type_info in TYPES_INFO.iter() {
        let namespace_super = T::namespace_super();

        let namespace_sub = T::namespace_sub();

        let class_ident = T::class_ident()
            .replace("<type_platform>", type_info.type_platform)
            .replace("<type_cs>", type_info.type_cs);

        let new_args = T::new_args()
            .replace("<type_platform>", type_info.type_platform)
            .replace("<type_cs>", type_info.type_cs);

        let new_expr = if type_info.has_new {
            if let Some(expr) = T::new_expr() {
                Some(
                    expr.replace("<type_platform>", type_info.type_platform)
                        .replace("<type_cs>", type_info.type_cs),
                )
            } else {
                None
            }
        } else {
            None
        };

        let drop_ident = T::drop_ident()
            .replace("<type_platform>", type_info.type_platform)
            .replace("<type_cs>", type_info.type_cs);

        let additional_methods = T::additional_methods().as_ref().map(|value| {
            value
                .replace("<type_platform>", type_info.type_platform)
                .replace("<type_cs>", type_info.type_cs)
        });

        output.push(ClassData {
            namespace_super,
            namespace_sub,
            class_ident,
            new_args,
            new_expr,
            drop_ident,
            additional_methods,
        });
    }

    output
}

pub struct Codegen {
    reg: Handlebars<'static>,
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
        // Once handlebars-rs properly handles multi-line partials, we will remove this line
        reg.set_prevent_indent(true);

        // https://docs.rs/handlebars/latest/handlebars/#strict-mode
        reg.set_strict_mode(false);
        // Don't escape characters
        reg.register_escape_fn(|s| s.to_string());

        reg.register_template_file(TPL_NAME, &tpl_path)
            .into_diagnostic()?;

        reg.register_partial("additional_methods", "{{additional_methods}}")
            .into_diagnostic()?;

        Ok(Self { reg, output_dir })
    }

    pub fn render_to_file(&self, data: &ClassData) -> Result<()> {
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
}
