fn main() {
    if std::env::var("RELEASE").is_ok() {
        std::env::set_var("PROTOC", "/usr/bin/protoc");
        tonic_build::configure()
            .compile(&[
                "proto/criteria.proto",
                "proto/profile.proto"
            ], &["proto"])
            .expect("unable to run protoc");
    } else {
        tonic_build::configure()
            .compile(&[
                "../../proto/criteria.proto",
                "../../proto/profile.proto"
            ], &["../../proto"])
            .expect("unable to run protoc");
    }
}