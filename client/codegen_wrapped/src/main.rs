mod class_data_templates;

use clap::Parser;
use cs_codegen::Codegen;
use miette::{Result, WrapErr};

use crate::class_data_templates::KeyframeGenerator;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;

    KeyframeGenerator::class_data()
        .iter()
        .map(|data| codegen.render_to_file(data))
        .collect()
}
