use std::any::Any;
use std::error::Error;

use tonic::{Code, Response, Status as GrpcStatus};

pub struct Status;

impl Status {
    pub fn unauthenticated(message: &str) -> Result<Response<dyn Any>, GrpcStatus> {
        Err(GrpcStatus::new(Code::Unauthenticated, message))
    }

    pub fn invalid_arguments(args: Vec<&str>) -> Result<Response<dyn Any>, GrpcStatus> {
        let message = format!("Invalid Arguments: {}", args.join(","));
        Err(GrpcStatus::new(Code::InvalidArgument, message))
    }

    pub fn internal(error: Box<dyn Error>) -> Result<Response<dyn Any>, GrpcStatus> {
        Err(GrpcStatus::new(Code::Internal, error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthenticated() {
        let status = Status::unauthenticated("test");
        assert_eq!(status.unwrap_err().message(), "test");
    }

    #[test]
    fn invalid_argument() {
        let status = Status::invalid_arguments(vec!["test"]);
        assert_eq!(status.unwrap_err().message(), "Invalid Arguments: test");
    }

    #[test]
    fn internal() {
        let error = std::fmt::Error;
        let status = Status::internal(Box::new(error));
        assert_eq!(status.unwrap_err().message(), error.to_string());
    }
}
