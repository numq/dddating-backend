pub mod service;

pub mod criteria {
    tonic::include_proto!("criteria");
}

pub mod pb {
    tonic::include_proto!("profile");
}