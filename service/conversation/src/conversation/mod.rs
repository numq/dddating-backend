pub mod interactor;
pub mod message_handler;
pub mod service;

pub mod pb {
    tonic::include_proto!("conversation");
}