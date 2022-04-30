use cs_codegen::{ClassDataTemplate, Codegen, KeyframeTemplate, StateTemplate};
use miette::{Result, WrapErr};

fn main() -> Result<()> {
    let codegen = Codegen::new().wrap_err("Failed to create `Codegen`")?;

    let class_data_templates = Vec::from([
        KeyframeTemplate::generate_class_data(),
        StateTemplate::generate_class_data(),
    ]);

    class_data_templates
        .iter()
        .map(|template| {
            template
                .iter()
                .map(|data| codegen.render_to_file(data))
                .collect()
        })
        .collect()
}
