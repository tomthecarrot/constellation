use eyre::{eyre, Result, WrapErr};
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

const TPL_NAME: &'static str = "tpl";

pub struct Codegen {
    reg: Handlebars<'static>,
    output_dir: PathBuf,
}

impl Codegen {
    pub fn new(partial_tpl_filename: &str) -> Result<Self> {
        let tpl_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(partial_tpl_filename);
        let partial_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(partial_tpl_filename);
        let output_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("cs/src/generated");

        if output_dir.exists() && output_dir.read_dir()?.next().is_some() {
            return Err(eyre!(format!(
                "`output_dir` is not empty! Please delete it. (output_dir={output_dir:?})"
            )));
        }
        std::fs::create_dir_all(&output_dir)
            .wrap_err_with(|| format!("Failed to create `output_dir`={output_dir:?}"))?;

        let mut reg = handlebars::Handlebars::new();
        // Once handlebars-rs properly handles multi-line partials, we will remove this line
        reg.set_prevent_indent(true);

        // Error when missing a value
        reg.set_strict_mode(true);

        // Don't escape characters
        reg.register_escape_fn(|s| s.to_string());

        reg.register_template_file(TPL_NAME, &tpl_path)?;

        std::fs::read_to_string(&partial_path).wrap_err("Failed to read partial template file")?;

        Ok(Self { reg, output_dir })
    }

    pub fn render_to_file(&self, type_info: &TypeInfo) -> Result<()> {
        let output_path = self
            .output_dir
            .join(format!("RBox_{}.cs", type_info.type_platform));
        let output_file = File::create(&output_path)
            .wrap_err_with(|| format!("Failed to create output file for RBox."))?;

        self.reg
            .render_to_write(TPL_NAME, type_info, output_file)
            .wrap_err("Failed to render to file")
    }

    pub fn render_all(&self) -> Result<()> {
        TYPES_INFO.iter().for_each(|type_info| {
            self.render_to_file(type_info)
                .expect("Failed to render to file");
        });
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
        // TypeInfo::new("ObjectHandle", "IntPtr", ""),
        // TypeInfo::new("ContractDataHandle", "IntPtr", ""),
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
