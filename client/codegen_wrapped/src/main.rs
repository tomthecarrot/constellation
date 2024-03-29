use cs_codegen::{CDKeyframe, CDState, CDStateHandle, CDStateId, ClassData, Codegen};

use clap::Parser;
use miette::{Result, WrapErr};

#[derive(Parser)]
struct Args {
    #[clap(short)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let codegen_keyframe =
        Codegen::new("keyframe.cs.tpl", args.force).wrap_err("Failed to create `Codegen`")?;
    let codegen_state =
        Codegen::new("state.cs.tpl", args.force).wrap_err("Failed to create `Codegen`")?;
    let codegen_state_id =
        Codegen::new("state_id.cs.tpl", args.force).wrap_err("Failed to create `Codegen`")?;
    let codegen_state_handle =
        Codegen::new("state_handle.cs.tpl", args.force).wrap_err("msFailed to create `Codegen`")?;

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

    let cd_state_id = ClassData::<CDStateId>::generate_class_data();
    let result_state_id: Result<()> = cd_state_id
        .iter()
        .map(|d| codegen_state_id.render_to_file(d))
        .collect();

    let cd_state_handle = ClassData::<CDStateHandle>::generate_class_data();
    let result_state_handle: Result<()> = cd_state_handle
        .iter()
        .map(|d| codegen_state_handle.render_to_file(d))
        .collect();

    let results = vec![
        result_keyframe,
        result_state,
        result_state_id,
        result_state_handle,
    ];

    results.into_iter().collect()
}
