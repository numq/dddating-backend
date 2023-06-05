pub mod entity;
pub mod mapper;
pub mod repository;
pub mod interactor;
pub mod service;

pub mod pb {
    tonic::include_proto!("support");
}