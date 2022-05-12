use cs_codegen::{ClassData, Codegen, Kf};
use miette::{Result, WrapErr};

fn main() -> Result<()> {
    let kf_codegen = Codegen::new("keyframe.cs.tpl").wrap_err("Failed to create `Codegen`")?;

    ClassData::<Kf>::generate_class_data()
        .iter()
        .map(|d| kf_codegen.render_to_file(d))
        .collect()
}
