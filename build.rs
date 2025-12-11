use prost_build::Config;
use protoc_bin_vendored::protoc_bin_path;

fn main() {
    // 获取自动内置的 protoc 路径
    let protoc_path = protoc_bin_path().expect("Failed to get protoc binary");
    
    // 设置 PROTOC 环境变量 —— 告诉 prost-build 使用内置 protoc
    std::env::set_var("PROTOC", protoc_path);
    
    let mut config = Config::new();
    config.compile_protos(&["proto/course.proto"], &["proto/"])
        .expect("Failed to compile protobuf files");
    
    println!("cargo:rerun-if-changed=proto/course.proto");
}
