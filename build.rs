fn main() {
    // 编译 Protobuf 文件到默认输出目录（OUT_DIR）
    prost_build::compile_protos(&["proto/course.proto"], &["proto/"])
        .expect("Failed to compile protobuf files");
    
    println!("cargo:rerun-if-changed=proto/course.proto");
}
