use std::sync::Arc;

use crate::{repository::models::TodoModel, service};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn todo(
    State(state): State<Arc<service::todo::Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Option<TodoModel>>) {
    match state.get_todo(id).await {
        Some(value) => (StatusCode::OK, Json(Some(value))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

pub async fn todos(
    State(state): State<Arc<service::todo::Service>>,
) -> (StatusCode, Json<Option<Vec<TodoModel>>>) {
    match state.get_list_todo().await {
        Some(v) => (StatusCode::OK, Json(Some(v))),
        None => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
