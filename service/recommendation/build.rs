fn main() {
    if std::env::var("RELEASE").is_ok() {
        std::env::set_var("PROTOC", "/usr/bin/protoc");
    } else {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    }
    tonic_build::configure()
        .compile(&["../../proto/criteria.proto", "../../proto/recommendation.proto"], &["../../proto"])
        .expect("unable to run protoc");
    tonic_build::configure()
        .build_client(true)
        .compile(&["../../proto/matchmaking.proto", "../../proto/profile.proto"], &["../../proto"])
        .expect("unable to run protoc");
}