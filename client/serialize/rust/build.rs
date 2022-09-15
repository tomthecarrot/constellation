use std::path::{Path, PathBuf};

fn main() -> flatc_rust::Result<()> {
    println!("cargo:rerun-if-changed=../flatbuffers");

    flatc_rust::run(flatc_rust::Args {
        inputs: &[&Path::new("../flatbuffers/all.fbs")],
        // out_dir: &Path::new("src/generated"),
        out_dir: PathBuf::from(format!("{}", std::env::var("OUT_DIR").unwrap())).as_path(),
        extra: &["--gen-all", "--rust-module-root-file"],
        ..Default::default()
    })?;

    Ok(())
}
