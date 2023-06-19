pub mod interactor;
pub mod amqp;
pub mod service;

pub mod pb {
    tonic::include_proto!("conversation");
}