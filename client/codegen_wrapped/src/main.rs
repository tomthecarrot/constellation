use cs_codegen::{Codegen, KeyframeTemplate};
use miette::{Result, WrapErr};

fn main() -> Result<()> {
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;

    KeyframeTemplate::new()
        .generate_class_data()
        .iter()
        .map(|data| codegen.render_to_file(data))
        .collect()
}
