use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::gateway::base::middleware::auth::TokenValidator;
use crate::gateway::base::{ApiResult, AppError, AppJson, ToApiResult};

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// 简单的登录接口：传入用户名，如果是 admin 则返回特定的 token
pub async fn handle_login(AppJson(req): AppJson<LoginRequest>) -> ApiResult<LoginResponse> {
    if req.username == "admin1" {
        LoginResponse {
            token: "gic-secret1".to_string(),
        }
        .ok()
    } else if req.username == "admin2" {
        LoginResponse {
            token: "gic-secret2".to_string(),
        }
        .ok()
    } else {
        Err(AppError::Unauthorized("Invalid username".into()))
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 简单实现：从 Authorization Header 中提取并假设它就是用户名
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

        // 这里只是模拟提取逻辑
        // 实际情况可能会解析 JWT 或查询数据库
        if token == "gic-secret1" {
            Ok(User {
                id: "1".into(),
                name: "Admin1".into(),
            })
        } else if token == "gic-secret2" {
            Ok(User {
                id: "2".into(),
                name: "Admin2".into(),
            })
        } else {
            // 如果 Token 不匹配，返回未授权错误
            Err(AppError::Unauthorized("Invalid token for user".into()))
        }
    }
}

#[derive(Clone)]
pub struct MyValidator;

impl TokenValidator for MyValidator {
    fn validate(&self, token: String) -> BoxFuture<'static, bool> {
        Box::pin(async move {
            // ── 这里是您的业务逻辑入口 ──
            // 您可以直接在这里进行异步数据库查询或 Redis 校验
            // 示例：模拟数据库查询延迟 (10ms)
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            // 目前为了测试，我们简单地将 token 与 "gic-secret" 进行比对
            // 实际使用时，您可以将其改为数据库查询结果
            token == "gic-secret1" || token == "gic-secret2"
        })
    }
}
