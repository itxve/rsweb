use axum::{body::Body, http::Request};
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};
use tracing::info;

#[derive(Clone)]
pub struct LoggingLayer;

impl<Svc> Layer<Svc> for LoggingLayer {
    type Service = LoggingMiddleware<Svc>;

    fn layer(&self, inner: Svc) -> Self::Service {
        LoggingMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LoggingMiddleware<Svc> {
    inner: Svc,
}

impl<Svc> Service<Request<Body>> for LoggingMiddleware<Svc>
where
    Svc: Service<Request<Body>, Response = axum::response::Response>,
{
    type Response = Svc::Response;
    type Error = Svc::Error;
    type Future = LoggingFuture<Svc::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let start_time = Instant::now();

        // Call the inner service
        let future = self.inner.call(req);

        LoggingFuture {
            future,
            method,
            uri,
            start_time,
        }
    }
}

pin_project! {
    pub struct LoggingFuture<F> {
        #[pin]
        future: F,
        method: axum::http::Method,
        uri: axum::http::Uri,
        start_time: Instant,
    }
}

impl<F, E> Future for LoggingFuture<F>
where
    F: Future<Output = Result<axum::response::Response, E>>,
{
    type Output = Result<axum::response::Response, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.future.poll(cx) {
            Poll::Ready(result) => {
                let duration = this.start_time.elapsed();

                match &result {
                    Ok(response) => {
                        info!(
                            method = %this.method,
                            uri = %this.uri,
                            status = %response.status(),
                            latency = ?duration,
                            "Request completed"
                        );
                    }
                    Err(_) => {
                        info!(
                            method = %this.method,
                            uri = %this.uri,
                            latency = ?duration,
                            "Request failed"
                        );
                    }
                }

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
