use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

mod get;
mod model;
mod post;

pub fn todo_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get::todos))
        .route("/", post(post::todo))
        .route("/{id}", get(get::todo))
}
