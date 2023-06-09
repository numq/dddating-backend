pub mod entity;
pub mod repository;
pub mod interactor;
pub mod service;

pub mod pb {
    tonic::include_proto!("safety");
}