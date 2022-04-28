use clap::Parser;
use cs_codegen::{ClassData, Codegen};
use miette::{IntoDiagnostic, Result, WrapErr};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;

    let data = ClassData {
        class_ident: "Keyframe_U8".to_string(),
        new_args: "byte value, double time".to_string(),
        new_expr: "generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8New(RSharp.RBox_U8.new_(value), time)".to_string(),
        drop_ident: "generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8Drop".to_string(),
        additional_methods: Some(r#"
            public unsafe byte Value
            {
                get
                {
                    byte* result = generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8Value(this.Ptr?.p ?? IntPtr.Zero);
                    return ToManaged.f(OwnershipSemantics.SharedRef, result);
                }
            }

            public double Time
            {
                get => generated.__Internal.TpClientContractPropertiesChannelsKeyframeU8Time(this.Ptr?.p ?? IntPtr.Zero);
            }
        "#.to_string()),
    };

    codegen.render(data)
}
