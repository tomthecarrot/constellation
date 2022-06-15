use handlebars::Handlebars;
use lazy_static::lazy_static;
use miette::{miette, ErrReport, IntoDiagnostic, Result, WrapErr};
use serde::Serialize;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const TPL_NAME: &'static str = "tpl";

pub struct Codegen {
    reg: Handlebars<'static>,
    output_path: PathBuf,
}

impl Codegen {
    pub fn new() -> Result<Self> {
        let tpl_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("rbox.cs.tpl");
        let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("cs/src/generated/wrapped");
        let output_path = output_dir.join("RBox.cs");

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

        Ok(Self { reg, output_path })
    }

    pub fn render(&self) -> Result<()> {
        let output_file = File::create(&self.output_path)
            .into_diagnostic()
            .wrap_err_with(|| format!("Failed to create output file for RBox."))?;

        let mut output_str = String::new();

        TYPES_INFO.iter().for_each(|item| {
            let result = self
                .reg
                .render(TPL_NAME, item)
                .into_diagnostic()
                .wrap_err("Failed to render to file");

            output_str.push_str(&result.unwrap());
        });

        println!("OUTPUT STR: {}", output_str);
        println!("OUTPUT PATH: {}", self.output_path.display());

        Ok(())
    }
}

lazy_static! {
    // Platform Type | C# Type
    static ref TYPES_INFO: Vec<TypeInfo> = Vec::from([
        TypeInfo::new("U8", "byte", "*"),
        TypeInfo::new("U16", "ushort", "*"),
        TypeInfo::new("U32", "uint", "*"),
        TypeInfo::new("U64", "ulong", "*"),
        TypeInfo::new("I8", "sbyte", "*"),
        TypeInfo::new("I16", "short", "*"),
        TypeInfo::new("I32", "int", "*"),
        TypeInfo::new("I64", "long", "*"),
        TypeInfo::new("Bool", "bool", "*"),
        TypeInfo::new("F32", "float", "*"),
        TypeInfo::new("F64", "double", "*"),
        TypeInfo::new("ObjectHandle", "IntPtr", ""),
        TypeInfo::new("ContractDataHandle", "IntPtr", ""),
    ]);
}

#[derive(Serialize)]
pub struct TypeInfo {
    type_platform: &'static str,
    type_cs: &'static str,
    ptr_literal: &'static str,
}
impl TypeInfo {
    fn new(type_platform: &'static str, type_cs: &'static str, ptr_literal: &'static str) -> Self {
        Self {
            type_platform,
            type_cs,
            ptr_literal,
        }
    }
}
