use crate::components::ApiError;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, Future, Ready};

use resalt_models::AuthStatus;
use std::pin::Pin;
use std::rc::Rc;

pub struct RequireAuth {}

impl RequireAuth {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequireAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireAuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct RequireAuthMiddleware<S: 'static> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            {
                // Check if req has extension AuthStatus{}
                match req.extensions().get::<AuthStatus>() {
                    Some(auth_status) => match auth_status.salt_token {
                        Some(_) => (),
                        None => {
                            return Err(ApiError::Unauthorized.into());
                        }
                    },
                    None => {
                        return Err(ApiError::Unauthorized.into());
                    }
                }
            }

            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        })
    }
}
