use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

use crate::gateway::base::ApiResponse;

/// 权限验证 Trait，允许用户自定义 Token 校验逻辑
pub trait TokenValidator: Send + Sync + 'static {
    fn validate(&self, token: String) -> BoxFuture<'static, bool>;
}

// The layer that will be applied to routes
#[derive(Clone)]
pub struct AuthLayer {
    validator: Arc<dyn TokenValidator>,
}

impl AuthLayer {
    pub fn new<V: TokenValidator>(validator: V) -> Self {
        Self {
            validator: Arc::new(validator),
        }
    }
}

impl<Svc> Layer<Svc> for AuthLayer {
    type Service = AuthMiddleware<Svc>;

    fn layer(&self, inner: Svc) -> Self::Service {
        AuthMiddleware {
            inner,
            validator: self.validator.clone(),
        }
    }
}

// The actual middleware service
#[derive(Clone)]
pub struct AuthMiddleware<Svc> {
    inner: Svc,
    validator: Arc<dyn TokenValidator>,
}

impl<Svc> Service<Request<Body>> for AuthMiddleware<Svc>
where
    Svc: Service<Request<Body>, Response = axum::response::Response> + Clone + Send + 'static,
    Svc::Future: Send,
{
    type Response = Svc::Response;
    type Error = Svc::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let validator = self.validator.clone();

        Box::pin(async move {
            // Extract the Authorization header
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // 使用自定义 Validator 校验
            let is_authorized = if let Some(token_str) = auth_header {
                let token = token_str
                    .strip_prefix("Bearer ")
                    .unwrap_or(&token_str)
                    .to_string();
                validator.validate(token).await
            } else {
                false
            };

            if is_authorized {
                // Token is valid, proceed to the inner service
                inner.call(req).await
            } else {
                // Token is missing or invalid, return 401
                let response = (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::<()>::error(
                        StatusCode::UNAUTHORIZED.as_u16(),
                        "Unauthorized".to_string(),
                    )),
                )
                    .into_response();
                Ok(response)
            }
        })
    }
}
