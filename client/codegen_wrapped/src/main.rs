use clap::Parser;
use cs_codegen::{ClassData, ClassDataTemplate, Codegen};
use miette::{Result, WrapErr};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;

    // At the moment there is only a `Keyframe` template.
    let templates = Vec::from([ClassDataTemplateKeyframe::template()]);

    let generated_class_data = codegen.monomorphize_templated_class_data(templates);
    let results = generated_class_data
        .iter()
        .map(|data| codegen.substitute_and_write_class_data(data))
        .collect();
    results
}

struct ClassDataTemplateKeyframe {}

impl ClassDataTemplate for ClassDataTemplateKeyframe {
    fn template() -> ClassData {
        ClassData {
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
}
