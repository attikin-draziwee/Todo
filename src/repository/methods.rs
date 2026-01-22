use sqlx::MySqlPool;

use crate::repository::models::{TodoCreate, TodoModel};

pub async fn list_todo(db: &MySqlPool) -> Result<Vec<TodoModel>, sqlx::Error> {
    sqlx::query_as!(
        TodoModel,
        "SELECT id, title, content, created, updated FROM todo"
    )
    .fetch_all(db)
    .await
}

pub async fn fetch_todo_by_id(id: u32, db: &MySqlPool) -> Result<TodoModel, sqlx::Error> {
    sqlx::query_as!(
        TodoModel,
        "SELECT id, title, content, created, updated FROM todo WHERE id = ?",
        id
    )
    .fetch_one(db)
    .await
}

pub async fn len_todo(db: &MySqlPool) -> Result<u64, sqlx::Error> {
    let count: u64 = sqlx::query_scalar("SELECT COUNT(*) FROM todo")
        .fetch_one(db)
        .await?;
    Ok(count)
}

pub async fn insert_todo(db: &MySqlPool, todo: TodoCreate) -> Result<TodoModel, sqlx::Error> {
    let todo_id = sqlx::query!(
        "INSERT INTO todo (title, content)
        VALUES (?, ?)",
        todo.get_title(),
        todo.get_content()
    )
    .execute(db)
    .await?
    .last_insert_id() as u32;
    Ok(TodoModel::new(
        todo_id,
        todo.get_title(),
        todo.get_content(),
    ))
}
