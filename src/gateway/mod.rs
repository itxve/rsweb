pub mod api;
pub mod base;
pub mod sse;
pub mod state;
pub mod static_files;
pub mod ws;

use crate::{error::AppError, gateway::state::IndexState};
use anyhow::Result;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

use axum::{
    body::Body,
    extract::Request,
    middleware::{self, Next},
    response::Response,
};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;

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
    // Build router with middleware
    let app = Router::new()
        // ── API routes ──
        .route("/api/health", get(api::handle_health))
        // ── SSE event stream ──
        .route("/api/events", get(sse::handle_sse_events))
        // ── WebSocket agent chat ──
        .route("/ws/chat", get(ws::handle_ws_chat))
        // ── API routes ──
        .route("/api/id", get(api::get_id))
        .route("/api/id_add", get(api::id_add))
        // ── API 404 catch-all ──
        .route(
            "/api/{*path}",
            get(|| async { AppError::NotFound("API route not found".into()) }),
        )
        // ── Static assets (web dashboard) ──
        .route("/_app/{*path}", get(static_files::handle_static))
        // ── Layers ──
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
