use tonic::{Request, Status};
use tonic::service::Interceptor;

#[derive(Copy, Clone)]
pub struct AuthorizationInterceptor;

impl AuthorizationInterceptor {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl Interceptor for AuthorizationInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let metadata = request.metadata();
        if let Some(token) = metadata.get("Authorization").and_then(|value| value.to_str().ok()) {
            todo!("process token");
            Ok(request)
        } else {
            Err(Status::invalid_argument("Error reading metadata"))
        }
    }
}