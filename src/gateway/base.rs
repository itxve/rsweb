use serde_json::error;
use thiserror::Error;

use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Json};
use serde::Serialize;

// =============================================================================
// 统一响应结构
// =============================================================================

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }
}

// =============================================================================
// 错误处理
// =============================================================================

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Internal Error: {0}")]
    Internal(String),

    #[error("Request Cancelled: {0}")]
    Cancelled(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, msg) = match self {
            AppError::BadRequest(m) => (axum::http::StatusCode::BAD_REQUEST, 400, m),
            AppError::NotFound(m) => (axum::http::StatusCode::NOT_FOUND, 404, m),
            AppError::Internal(m) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, 500, m),
            AppError::Cancelled(m) => (axum::http::StatusCode::OK, 1, m),
            AppError::Conflict(m) => (axum::http::StatusCode::CONFLICT, 409, m),
        };

        (status, Json(ApiResponse::<()>::error(code, msg))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
pub type Res<T> = AppResult<AppJson<T>>;

// =============================================================================
// 自定义 Json 提取器
// =============================================================================

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> From<T> for AppJson<T> {
    fn from(t: T) -> Self {
        AppJson(t)
    }
}

pub trait ToRes: Sized {
    fn ok(self) -> Res<Self> {
        Ok(AppJson(self))
    }
}
impl<T: Serialize> ToRes for T {}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::BadRequest(rejection.body_text())
    }
}

impl<T> IntoResponse for AppJson<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(ApiResponse::success(self.0)).into_response()
    }
}
