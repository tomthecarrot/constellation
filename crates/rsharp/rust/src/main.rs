use miette::{Result, WrapErr};
use rsharp::codegen::Codegen;

fn main() -> Result<()> {
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;
    codegen.render()
}
