pub mod entity;
pub mod mapper;
pub mod api;

pub mod criteria {
    tonic::include_proto!("criteria");
}

pub mod pb {
    tonic::include_proto!("profile");
}