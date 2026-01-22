pub mod todo;

use axum::Json;
use serde_json::{Value, json};

pub async fn hello_api() -> Json<Value> {
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
