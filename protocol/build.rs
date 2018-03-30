extern crate protoc_rust_grpc;

fn main() {
    if true {
        return;
    }
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &["proto"],
        input: &["proto/datasquid.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");
}
