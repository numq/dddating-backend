pub mod mapper;
pub mod repository;
pub mod interactor;
pub mod amqp;
pub mod service;

pub mod criteria {
    tonic::include_proto!("criteria");
}

pub mod pb {
    tonic::include_proto!("recommendation");
}