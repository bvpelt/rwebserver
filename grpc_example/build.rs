use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=foobar.proto");

    // Get the path to the grpc_example directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Point it directly into your src/ directory
    let out_dir = PathBuf::from(manifest_dir).join("src");

    tonic_prost_build::configure()
        .out_dir(out_dir)
        .build_server(true)
        .build_client(true)
        .compile_protos(&["foobar.proto"], &["."])?;

    Ok(())
}
