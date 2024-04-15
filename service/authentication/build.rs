fn main() {
    if std::env::var("RELEASE").is_ok() {
        std::env::set_var("PROTOC", "/usr/bin/protoc");
    } else {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    }
    tonic_build::compile_protos("../../proto/authentication.proto").expect("unable to run protoc");
    tonic_build::configure()
        .build_client(true)
        .compile(&["../../proto/account.proto", "../../proto/token.proto"], &["../../proto"])
        .expect("unable to run protoc");
}