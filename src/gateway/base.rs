use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Json};
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

// =============================================================================
// 统一响应 JSON 结构 (最终发给前端)
// =============================================================================

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn error(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }
}

// =============================================================================
// 响应包装器 (用于 Handler 返回成功数据)
// =============================================================================

pub struct Reply<T>(pub T, pub i32);

impl<T> IntoResponse for Reply<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let body = ApiResponse {
            code: self.1,
            msg: "success".to_string(),
            data: Some(self.0),
        };
        axum::Json(body).into_response()
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

// Handler 的标准返回类型
pub type ApiResult<T> = Result<Reply<T>, AppError>;

// =============================================================================
// 输入提取器 (仅用于从 Request 提取 JSON)
// =============================================================================

pub struct AppJson<T>(pub T);

impl<T, S> FromRequest<S> for AppJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(axum::Json(data)) => Ok(AppJson(data)),
            Err(rejection) => Err(AppError::from(rejection)),
        }
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::BadRequest(rejection.body_text())
    }
}

// =============================================================================
// 便捷转换 Trait
// =============================================================================

pub trait ToApiResult: Sized {
    fn ok(self) -> ApiResult<Self> {
        Ok(Reply(self, 0))
    }
    fn with_code(self, code: i32) -> ApiResult<Self> {
        Ok(Reply(self, code))
    }
}

impl<T: Serialize> ToApiResult for T {}
