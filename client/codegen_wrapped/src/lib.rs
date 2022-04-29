use handlebars::Handlebars;
use miette::{miette, IntoDiagnostic, Result, WrapErr};
use serde::Serialize;
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[macro_use]
extern crate lazy_static;

const TPL_NAME: &'static str = "tpl";

#[derive(Clone, Serialize)]
pub struct ClassData {
    pub class_ident: String,
    pub new_args: String,
    pub new_expr: String,
    pub drop_ident: String,
    pub additional_methods: Option<String>,
}

lazy_static! {
    static ref TYPES: Vec<(&'static str, &'static str, &'static str)> = Vec::from([
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

    pub fn render(&self, data: &ClassData) -> Result<()> {
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

    pub fn render_vec(&self, data_vec: Vec<ClassData>) -> Result<()> {
        for data in data_vec.iter() {
            let res = self.render(data);
            if let Err(report) = res {
                return Err(report);
            }
        }

        Ok(())
    }

    pub fn render_class_data(&self) -> Result<()> {
        self.render_vec(self.generate_class_data())
    }

    fn generate_templates(&self) -> Vec<ClassData> {
        let mut templates: Vec<ClassData> = Vec::new();

        let template_keyframe = ClassData {
            class_ident: "Keyframe_<type_platform>".to_string(),
            new_args: "<type_cs> value, double time".to_string(),
            new_expr: "generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>New(RSharp.RBox_<type_platform>.new_(value), time)".to_string(),
            drop_ident: "generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Drop".to_string(),
            additional_methods: Some(r#"
                public unsafe <type_cs> Value
                {
                    get
                    {
                        <type_cs_ptr> result = generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Value(this.Ptr?.p ?? IntPtr.Zero);
                        return ToManaged.f(OwnershipSemantics.SharedRef, result);
                    }
                }

                public double Time
                {
                    get => generated.__Internal.TpClientContractPropertiesChannelsKeyframe<type_platform>Time(this.Ptr?.p ?? IntPtr.Zero);
                }
            "#.to_string()),
        };
        templates.push(template_keyframe);

        templates
    }

    pub fn generate_class_data(&self) -> Vec<ClassData> {
        let templates = self.generate_templates();

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
