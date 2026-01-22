use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::todo;

mod get;
mod model;
mod post;

pub fn todo_router() -> Router {
    Router::new()
        .route("/", get(todo::get::todos))
        .route("/", post(todo::post::todo))
        .route("/{id}", get(todo::get::todo))
}
