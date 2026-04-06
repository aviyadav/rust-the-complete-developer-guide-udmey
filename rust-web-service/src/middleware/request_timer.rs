use std::future::{ready, Ready};
use std::time::Instant;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;

/// Middleware that logs the method, path, response status, and elapsed time
/// for every request.
pub struct RequestTimer;

impl<S, B> Transform<S, ServiceRequest> for RequestTimer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestTimerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestTimerMiddleware { service }))
    }
}

pub struct RequestTimerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestTimerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start.elapsed();
            log::info!(
                "{} {} → {} ({}ms)",
                method,
                path,
                res.status().as_u16(),
                elapsed.as_millis()
            );
            Ok(res)
        })
    }
}
