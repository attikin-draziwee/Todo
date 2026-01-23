use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    repository::models::{TodoModel, TodoPatch},
    service::{self, todo::Error},
};

pub async fn todo(
    State(state): State<Arc<service::todo::Service>>,
    Path(id): Path<u32>,
    Json(request): Json<TodoPatch>,
) -> (StatusCode, Json<Option<TodoModel>>) {
    match state.patch_todo(id, request).await {
        Ok(request) => (StatusCode::OK, Json(Some(request))),
        Err(err) => (
            match err {
                Error::BadRequest => StatusCode::BAD_REQUEST,
                Error::MySQLError(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(None),
        ),
    }
}
