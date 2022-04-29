use clap::Parser;
use cs_codegen::{ClassData, Codegen};
use miette::{Result, WrapErr};

#[derive(Parser)]
#[clap(version, about)]
struct Cli {}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;
    codegen.render_class_data()
}
