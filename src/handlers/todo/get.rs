use crate::{AppState, repository::models::TodoDataBase};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::repository;

pub async fn todo(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Option<TodoDataBase>>) {
    match repository::models::fetch_todo_by_id(id, &state.db).await {
        Ok(todo) => (StatusCode::OK, Json(Some(todo))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}

#[axum::debug_handler]
pub async fn todos(State(state): State<AppState>) -> (StatusCode, Json<Option<Vec<TodoDataBase>>>) {
    match repository::models::list_todo(&state.db).await {
        Ok(v) => (StatusCode::OK, Json(Some(v))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
