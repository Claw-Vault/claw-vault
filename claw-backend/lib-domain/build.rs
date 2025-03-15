use std::{env, path::PathBuf};

fn main() {
    println!("Compiling protos ...");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to get OUT_DIR"));

    tonic_build::configure()
        // .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .file_descriptor_set_path(out_dir.join("claw_descriptor.bin"))
        .compile_protos(&["../../proto/claw.proto"], &["../../proto"])
        .expect("Failed to compile service protos");
}
