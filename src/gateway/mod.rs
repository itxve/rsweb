pub mod api;

pub mod base;

pub mod sse;
pub mod state;
pub mod user;
pub mod ws;

use crate::gateway::{base::AppError, state::IndexState};
use anyhow::Result;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};

use axum::{
    extract::Request,
    middleware::{self, Next},
    response::Response,
};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;

use base::middleware::*;
use base::static_files;

/// 统一错误响应中间件
/// 确保所有 4xx/5xx 响应都符合统一的 JSON 格式
async fn error_unify_middleware(req: Request, next: Next) -> Response {
    let response = next.run(req).await;
    let status = response.status();

    if status.is_client_error() || status.is_server_error() {
        // 检查是否已经是 JSON 格式
        let is_json = response
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.contains("application/json"))
            .unwrap_or(false);

        if !is_json {
            // 如果不是 JSON，则转换成统一的 JSON 格式
            // 尝试获取原始错误信息，或者使用状态码默认描述
            let error_message = match status {
                StatusCode::NOT_FOUND => "Resource not found".to_string(),
                StatusCode::PAYLOAD_TOO_LARGE => "Request body too large".to_string(),
                StatusCode::REQUEST_TIMEOUT => "Request timeout".to_string(),
                _ => status
                    .canonical_reason()
                    .unwrap_or("Unknown error")
                    .to_string(),
            };

            let body = serde_json::json!({
                "error": error_message,
            });

            return (status, Json(body)).into_response();
        }
    }

    response
}

/// Maximum request body size (64KB) — prevents memory exhaustion
pub const MAX_BODY_SIZE: usize = 65_536;
/// Request timeout (30s) — prevents slow-loris attacks
pub const REQUEST_TIMEOUT_SECS: u64 = 30;

pub async fn run_gateway(host: &str, port: u16) -> Result<()> {
    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let state = IndexState::new();
    let validator = user::MyValidator;

    // ── Public API routes ──
    let public_routes = Router::new()
        .route("/health", get(api::handle_health))
        .route("/events", get(sse::handle_sse_events))
        .route("/user/login", post(user::handle_login));

    // ── Protected API routes ──
    let api_routes = Router::new()
        .route("/id", get(api::get_id))
        .route("/id_add", get(api::id_add))
        .route("/user", get(api::handle_user_info))
        .route("/test/json", post(api::handle_test_json))
        .route(
            "/{*path}",
            get(|| async { AppError::NotFound("API route not found".into()) }),
        )
        .layer(auth::AuthLayer::new(validator.clone()));

    // ── WebSocket agent chat (also protected) ──
    let ws_routes = Router::new()
        .route("/chat", get(ws::handle_ws_chat))
        .layer(auth::AuthLayer::new(validator));

    // Build router with middleware
    let app = Router::new()
        // ── Public routes ──
        .nest("/api", public_routes)
        // nest() 的关键特性
        // 自动加前缀
        // 中间件会影响所有子路由！
        .nest("/api", api_routes)
        .nest("/ws", ws_routes)
        // ── Static assets (web dashboard) ──
        .route("/_app/{*path}", get(static_files::handle_static))
        // ── Layers ──
        .layer(log::LoggingLayer)
        .layer(middleware::from_fn(error_unify_middleware))
        .layer(RequestBodyLimitLayer::new(MAX_BODY_SIZE))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(REQUEST_TIMEOUT_SECS),
        ))
        .with_state(state)
        // ── SPA fallback ──
        .fallback(get(static_files::handle_spa_fallback));

    // Run the server
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
