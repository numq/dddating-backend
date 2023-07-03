fn main() {
    std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    tonic_build::configure()
        .build_client(true)
        .compile(&[
            "../../proto/authentication.proto",
            "../../proto/conversation.proto",
            "../../proto/matchmaking.proto",
            "../../proto/profile.proto",
            "../../proto/recommendation.proto",
            "../../proto/safety.proto",
            "../../proto/support.proto"
        ], &["../../proto"])
        .expect("unable to run protoc");
}