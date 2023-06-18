fn main() {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    tonic_build::compile_protos("../../proto/conversation.proto").expect("unable to run protoc");
}