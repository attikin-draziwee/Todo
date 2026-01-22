use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::service::todo::Service;

mod get;
mod post;

pub fn todo_router() -> Router<Arc<Service>> {
    Router::new()
        .route("/", get(get::todos))
        .route("/", post(post::todo))
        .route("/{id}", get(get::todo))
}
