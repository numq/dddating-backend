pub mod entity;
pub mod mapper;
pub mod interactor;
pub mod repository;
pub mod service;

pub mod pb {
    tonic::include_proto!("account");
}