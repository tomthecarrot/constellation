use eyre::{Result, WrapErr};
use rsharp::codegen::Codegen;

fn main() -> Result<()> {
    let codegen = Codegen::new("rbox.cs.tpl").wrap_err("Failed to create `Codegen`")?;
    codegen.render_all()
}
