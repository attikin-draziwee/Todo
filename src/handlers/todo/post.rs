use axum::{Json, extract::State, http::StatusCode};

use crate::{AppState, handlers::todo::model::TodoCreate, repository::models::count_todos};

pub async fn todo(State(state): State<AppState>, Json(todo): Json<TodoCreate>) -> StatusCode {
    let count = match count_todos(&state.db).await {
        Ok(c) => (c + 1) as u32,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    let result = sqlx::query!(
        "INSERT INTO todo (id, title, content)
        VALUES (?, ?, ?)",
        count,
        todo.get_title(),
        todo.get_content()
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            eprintln!("Error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
