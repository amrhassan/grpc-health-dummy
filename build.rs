fn main() {
    tonic_build::compile_protos("proto/health.proto").unwrap();
}
