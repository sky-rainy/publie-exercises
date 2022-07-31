use std::path::PathBuf;
fn main() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dir_path = config_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("protobuf");

    // let tmp_path = config_path.join("tmp");
    // fs::create_dir_all(&tmp_path).unwrap();
    // fs::copy(dir_path.join("fts.proto"), tmp_path.join("fts.proto")).unwrap();

    let mut config = prost_build::Config::new();
    config.out_dir("src");

    config
        .compile_protos(&[dir_path.join("fts.proto")], &[dir_path])
        .unwrap();
}
