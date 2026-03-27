use axum::{extract::State, response::IntoResponse, Json};

use crate::gateway::{
    base::{Res, ToRes},
    state::IndexState,
};

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

pub async fn get_id(State(state): State<IndexState>) -> Res<serde_json::Value> {
    let dir = state.id.lock().unwrap();
    serde_json::json!({ "id": *dir }).ok()
}

pub async fn id_add(State(state): State<IndexState>) -> Res<serde_json::Value> {
    let mut dir = state.id.lock().unwrap();
    *dir += 1;
    serde_json::json!({ "id": *dir }).ok()
}
