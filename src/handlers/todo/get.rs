use crate::{AppState, handlers::todo::model::Todo, repository::models::TodoDataBase};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::repository;

pub async fn todo(Path(id): Path<usize>) -> (StatusCode, Json<Todo>) {
    (
        StatusCode::OK,
        Json(Todo::new(
            id,
            "Hello",
            "Hello, World! What can I do for you?",
        )),
    )
}

#[axum::debug_handler]
pub async fn todos(State(state): State<AppState>) -> (StatusCode, Json<Option<Vec<TodoDataBase>>>) {
    match repository::models::list_todo(&state.db).await {
        Ok(v) => (StatusCode::OK, Json(Some(v))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
