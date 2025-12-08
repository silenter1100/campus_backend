fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    }
    tonic_build::configure()
        .compile_protos(
            &["proto/forum.proto"], 
            &["proto"],
        )?;
    Ok(())
}