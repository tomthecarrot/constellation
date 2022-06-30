use cs_codegen::{CDKeyframe, CDState, ClassData, Codegen};
use miette::{Result, WrapErr};

fn main() -> Result<()> {
    let codegen_keyframe =
        Codegen::new("keyframe.cs.tpl").wrap_err("Failed to create `Codegen`")?;
    let codegen_state = Codegen::new("state.cs.tpl").wrap_err("Failed to create `Codegen`")?;

    let cd_keyframe = ClassData::<CDKeyframe>::generate_class_data();
    let result_keyframe: Result<()> = cd_keyframe
        .iter()
        .map(|d| codegen_keyframe.render_to_file(d))
        .collect();

    let cd_state = ClassData::<CDState>::generate_class_data();
    let result_state: Result<()> = cd_state
        .iter()
        .map(|d| codegen_state.render_to_file(d))
        .collect();

    let results = vec![result_keyframe, result_state];

    results.into_iter().collect()
}
