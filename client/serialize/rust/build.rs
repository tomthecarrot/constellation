use std::path::{Path, PathBuf};

fn main() -> flatc_rust::Result<()> {
    println!("cargo:rerun-if-changed=../flatbuffers");
    let flatc = flatc_rust::Flatc::from_env_path();
    assert_eq!(
        flatc.version().unwrap().version(),
        "23.1.21",
        "flatc was wrong version."
    );

    flatc.run(flatc_rust::Args {
        inputs: &[&Path::new("../flatbuffers/all.fbs")],
        out_dir: PathBuf::from(format!("{}", std::env::var("OUT_DIR").unwrap())).as_path(),
        extra: &["--gen-all", "--rust-module-root-file"],
        ..Default::default()
    })?;

    Ok(())
}
