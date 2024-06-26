use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tonic::{Request, Status};
use tonic_async_interceptor::{async_interceptor, AsyncInterceptedService, AsyncInterceptor};
use tower::ServiceBuilder;

use crate::authentication::interactor::AuthenticationInteractor;

#[derive(Clone)]
pub struct AuthInterceptor {
    interactor: Arc<Box<dyn AuthenticationInteractor + Send + Sync>>,
}

impl AsyncInterceptor for AuthInterceptor {
    type Future = Pin<Box<dyn Future<Output=Result<Request<()>, Status>> + Send + 'static>>;

    fn call(&mut self, request: Request<()>) -> Self::Future {
        let interactor = self.interactor.clone();

        let future = async move {
            let token = request.metadata().get("Authorization").and_then(|value| value.to_str().ok());
            match token {
                Some(token) => {
                    if let Ok(_) = interactor.is_token_valid(token).await {
                        Ok(request)
                    } else {
                        Err(Status::unauthenticated("Invalid token"))
                    }
                }
                None => Err(Status::invalid_argument("Authorization token is missing"))
            }
        };

        Box::pin(future)
    }
}

pub fn with_auth_interceptor<S>(
    service: S,
    interactor: Arc<Box<dyn AuthenticationInteractor + Send + Sync>>,
) -> AsyncInterceptedService<S, AuthInterceptor>
    where
        S: Clone,
        AuthInterceptor: AsyncInterceptor
{
    ServiceBuilder::new()
        .layer(async_interceptor(AuthInterceptor { interactor }))
        .service(service)
}