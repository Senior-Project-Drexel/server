fn main() {
    println!("cargo:rerun-if-changed=matrix_proto.proto");
    prost_build::compile_protos(&["matrix_proto.proto"], &["."])
        .expect("Failed to compile protos");
}