mod template_keyframe;
mod template_state;

pub use self::template_keyframe::CDKeyframe;
pub use self::template_state::CDState;

use handlebars::Handlebars;
use lazy_static::lazy_static;
use miette::{miette, IntoDiagnostic, Result, WrapErr};
use serde::Serialize;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const TPL_NAME: &'static str = "tpl";

/// `M` Is the additional data to populate `additional_methods` partial template
#[derive(Clone, Serialize)]
pub struct ClassData<M: Serialize = ()> {
    pub namespace_super: String,
    pub namespace_sub: String,
    pub class_ident: String,
    pub new_args: String,
    pub new_expr: Option<String>,
    pub drop_ident: Option<String>,
    #[serde(flatten)]
    pub additional_methods: Option<M>,
}

pub struct Codegen {
    reg: Handlebars<'static>,
    output_dir: PathBuf,
}
impl Codegen {
    pub fn new(partial_tpl_filename: &str) -> Result<Self> {
        let tpl_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("opaque_template.cs.tpl");
        let partial_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(partial_tpl_filename);
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

        // Error when missing a value
        reg.set_strict_mode(true);

        // Don't escape characters
        reg.register_escape_fn(|s| s.to_string());

        reg.register_template_file(TPL_NAME, &tpl_path)
            .into_diagnostic()?;

        let partial = std::fs::read_to_string(&partial_path)
            .into_diagnostic()
            .wrap_err("Faild to read partial template file")?;
        reg.register_partial("additional_methods", &partial)
            .into_diagnostic()?;

        Ok(Self { reg, output_dir })
    }

    pub fn render_to_file<M: Serialize>(&self, data: &ClassData<M>) -> Result<()> {
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
