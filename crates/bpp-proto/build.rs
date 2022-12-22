use std::path::PathBuf;

fn main() {
    tonic_build::configure()
        .out_dir(&PathBuf::from("src"))
        .compile(&["protos/api.proto"], &["protos"])
        .expect("Failed to compile protobuf")
}
