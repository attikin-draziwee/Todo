use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use serde_json::{Value, json};
use std::sync::Arc;

use crate::service;
use service::todo::Error;

pub async fn todo(
    State(state): State<Arc<service::todo::Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Value>) {
    match state.delete_todo(id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"status": "success"}))),
        Err(err) => {
            let status_code: StatusCode = match err {
                Error::MySQLError(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (
                status_code,
                Json(json!({
                    "status": "error",
                    "message": format!("{err:#?}")
                })),
            )
        }
    }
}
