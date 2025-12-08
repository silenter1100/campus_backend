use std::io::Result;

fn main() -> Result<()> {
    // 1. 设置 PROTOC 环境变量
    unsafe {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    }

    let mut config = prost_build::Config::new();

    // 2. 添加 Serde 序列化支持
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    // 3. 编译 Proto 文件
    config.compile_protos(
        &["proto/forum.proto"],
        &["proto"])?;

    Ok(())
}