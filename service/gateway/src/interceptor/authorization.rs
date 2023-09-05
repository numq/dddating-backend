use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tonic::{Request, Response, Status};
use tower::{Service, ServiceBuilder};

use interceptor::{async_interceptor, AsyncInterceptedService, AsyncInterceptor};

use crate::authentication::interactor::AuthenticationInteractor;

#[derive(Clone)]
pub struct AuthInterceptor<S> {
    service: S,
    interactor: Arc<Box<dyn AuthenticationInteractor + Send + Sync>>,
}

impl<S> AuthInterceptor<S> {
    pub fn new(service: S, interactor: Arc<Box<dyn AuthenticationInteractor + Send + Sync>>) -> Self {
        AuthInterceptor { service, interactor }
    }
}

impl<S> AsyncInterceptor for AuthInterceptor<S> {
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
) -> AsyncInterceptedService<S, AuthInterceptor<S>>
    where
        S: Clone,
        AuthInterceptor<S>: AsyncInterceptor
{
    ServiceBuilder::new()
        .layer(async_interceptor(AuthInterceptor { service: service.clone(), interactor }))
        .service(service)
}