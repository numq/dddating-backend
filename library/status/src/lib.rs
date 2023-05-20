use std::error::Error;

use tonic::{Code, Status as GrpcStatus};

pub struct Status;

impl Status {
    pub fn unauthenticated(message: &str) -> GrpcStatus {
        GrpcStatus::new(Code::Unauthenticated, message)
    }

    pub fn invalid_argument(message: &str) -> GrpcStatus {
        GrpcStatus::new(Code::InvalidArgument, message)
    }

    pub fn internal(error: Box<dyn Error>) -> GrpcStatus {
        GrpcStatus::new(Code::Internal, error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthenticated() {
        let status = Status::unauthenticated("test");
        assert_eq!(status.message(), "test");
    }

    #[test]
    fn invalid_argument() {
        let status = Status::invalid_argument("test");
        assert_eq!(status.message(), "test");
    }

    #[test]
    fn internal() {
        let error = std::fmt::Error;
        let status = Status::internal(Box::new(error));
        assert_eq!(status.message(), error.to_string());
    }
}
