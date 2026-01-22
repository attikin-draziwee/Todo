use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::{
    repository::models::{TodoCreate, TodoModel},
    service::todo::Service,
};

#[axum::debug_handler]
pub async fn todo(
    State(service): State<Arc<Service>>,
    Json(todo): Json<TodoCreate>,
) -> (StatusCode, Json<Option<TodoModel>>) {
    match service.create_todo(todo).await {
        Ok(todo) => (StatusCode::CREATED, Json(Some(todo))),
        Err(err) => {
            eprintln!("Failed to create todo: {err:#?}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}
