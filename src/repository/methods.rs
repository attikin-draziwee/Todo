use sqlx::MySqlPool;

use crate::repository::models::{TodoCreate, TodoModel, TodoPut};

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

pub async fn delete_todo(db: &MySqlPool, id: u32) -> Result<(), sqlx::Error> {
    match sqlx::query!(
        "
        DELETE FROM todo
        WHERE id = ?
        ",
        id
    )
    .execute(db)
    .await
    {
        Ok(value) => match value.rows_affected() {
            0 => Err(sqlx::Error::RowNotFound),
            _ => Ok(()),
        },
        Err(err) => Err(err),
    }
}

pub async fn patch_todo(
    db: &MySqlPool,
    id: u32,
    title: Option<String>,
    content: Option<String>,
) -> Result<TodoModel, sqlx::Error> {
    let result = sqlx::query!(
        "
        UPDATE todo 
        SET 
            title = COALESCE(?, title),
            content = COALESCE(?, content)
        WHERE id = ?
        ",
        title,
        content,
        id
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    fetch_todo_by_id(id, db).await
}

pub async fn put_todo(db: &MySqlPool, id: u32, todo: TodoPut) -> Result<TodoModel, sqlx::Error> {
    let result = sqlx::query!(
        "
        UPDATE todo 
        SET 
            title = ?,
            content = ?
        WHERE id = ?
        ",
        todo.title,
        todo.content,
        id
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    fetch_todo_by_id(id, db).await
}
