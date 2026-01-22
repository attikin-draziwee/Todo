use std::net::SocketAddr;

use axum::routing::get;
use axum::{Json, Router};

use tower_http::services::ServeDir;

use serde_json::{Value, json};

mod handlers;

use crate::handlers::todo::todo_router;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip_addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    let listener = tokio::net::TcpListener::bind(ip_addr).await?;

    axum::serve(listener, app()).await?;

    Ok(())
}

fn app() -> Router {
    let static_folder = ServeDir::new("static/");
    Router::new()
        .nest_service(
            "/api",
            Router::new()
                .route("/", get(hello_api))
                .route("/health", get(health))
                .nest_service("/todo", todo_router()),
        )
        .fallback_service(static_folder)
}

async fn hello_api() -> Json<Value> {
    Json(json!({
        "status" : "ok",
        "message" : "Hello, World!"
    }))
}

pub async fn health() -> Json<Value> {
    Json(json!(
        {
            "status": "ok",
            "message": "it is working!"
        }
    ))
}
