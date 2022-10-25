mod codegen;

use clap::Parser;
use eyre::{Result, WrapErr};

use crate::codegen::Codegen;

#[derive(Parser)]
struct Args {
    #[clap(short)]
    /// Forcibly overwrites existing files
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let codegen = Codegen::new(args.force).wrap_err("Failed to create `Codegen`")?;
    codegen.render_all()
}
