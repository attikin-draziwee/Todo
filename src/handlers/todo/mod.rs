use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::service::todo::Service;

mod delete;
mod get;
mod patch;
mod post;
mod put;

pub fn todo_router() -> Router<Arc<Service>> {
    Router::new()
        .route("/", get(get::todos))
        .route("/", post(post::todo))
        .route("/{id}", get(get::todo))
        .route("/{id}", delete(delete::todo))
        .route("/{id}", patch(patch::todo))
        .route("/{id}", put(put::todo))
}
