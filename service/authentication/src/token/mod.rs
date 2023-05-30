pub mod repository;
pub mod api;

pub mod pb {
    tonic::include_proto!("token");
}