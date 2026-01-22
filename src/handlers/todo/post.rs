use axum::{Json, http::StatusCode};

use crate::handlers::todo::model::{Todo, TodoCreate};

pub async fn todo(Json(todo): Json<TodoCreate>) -> StatusCode {
    let data: Todo = Todo::new(0, todo.get_title(), todo.get_content());
    println!("Raw data: {todo:?}");
    println!("Data in DB: {data:?}");
    StatusCode::CREATED
}
