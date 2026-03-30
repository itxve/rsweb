use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;

use crate::gateway::{
    base::{ApiResult, AppJson, ToApiResult},
    state::IndexState,
};

#[derive(Deserialize)]
pub struct TestJsonRequest {
    pub name: String,
    pub age: Option<u32>,
}

/// 测试 AppJson 提取器
pub async fn handle_test_json(
    AppJson(req): AppJson<TestJsonRequest>,
) -> ApiResult<serde_json::Value> {
    serde_json::json!({
        "received_name": req.name,
        "received_age": req.age,
        "message": "AppJson extractor works!"
    })
    .ok()
}

/// GET /health — always public (no secrets leaked)
pub async fn handle_health() -> impl IntoResponse {
    let body = serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": {
            "status": "ok"
        }
    });
    Json(body)
}

pub async fn get_id(State(state): State<IndexState>) -> ApiResult<serde_json::Value> {
    let dir = state.id.lock().unwrap();
    serde_json::json!({ "id": *dir }).ok()
}

pub async fn id_add(State(state): State<IndexState>) -> ApiResult<serde_json::Value> {
    let mut dir = state.id.lock().unwrap();
    *dir += 1;
    serde_json::json!({ "id": *dir }).ok()
}
