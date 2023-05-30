pub mod entity;
pub mod mapper;
pub mod interactor;
pub mod service;

pub mod pb {
    tonic::include_proto!("authentication");
}