use crate::handlers::todo::model::Todo;

use axum::{Json, extract::Path, http::StatusCode};

pub async fn todo(Path(id): Path<usize>) -> (StatusCode, Json<Todo>) {
    (
        StatusCode::OK,
        Json(Todo::new(
            id,
            "Hello",
            "Hello, World! What can I do for you?",
        )),
    )
}

pub async fn todos() -> (StatusCode, Json<Vec<Todo>>) {
    (
        StatusCode::OK,
        Json(
            (0..=5)
                .map(|el| Todo::new(el as usize, &format!("Todo#{el}"), "some content, y'now"))
                .collect::<Vec<Todo>>(),
        ),
    )
}
