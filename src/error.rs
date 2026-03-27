use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error;

/// 统一的 API 响应结构
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应 (code = 0)
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    /// 错误响应
    pub fn error(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }
}

/// 实现 IntoResponse 以便直接在 Axum 路由中返回
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// 统一错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),
}

/// 方便 API 开发的 Result 别名
pub type AppResult<T> = Result<ApiResponse<T>, AppError>;

impl AppError {
    pub fn code(&self) -> i32 {
        match self {
            AppError::Internal(_) => 500,
            AppError::NotFound(_) => 404,
            AppError::BadRequest(_) => 400,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::Internal(ref e) => {
                tracing::error!("Internal error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        let body: ApiResponse<Value> = ApiResponse::error(self.code(), self.to_string());

        (status, Json(body)).into_response()
    }
}

/// 方便将 std::io::Error 转换为 AppError
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(anyhow::anyhow!(err))
    }
}
