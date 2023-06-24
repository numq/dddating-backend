fn main() {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    tonic_build::compile_protos("../../proto/criteria.proto").expect("unable to run protoc");
    tonic_build::compile_protos("../../proto/recommendation.proto").expect("unable to run protoc");
    tonic_build::configure()
        .build_client(true)
        .compile(&["../../proto/matchmaking.proto", "../../proto/profile.proto"], &["../../proto"])
        .expect("unable to run protoc");
}