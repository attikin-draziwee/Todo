use ::chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, prelude::FromRow, types::chrono};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TodoDataBase {
    id: u32,
    title: String,
    content: String,
    created: chrono::NaiveDate,
    updated: chrono::NaiveDate,
}

impl TodoDataBase {
    pub fn new(id: u32, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            created: Utc::now().date_naive(),
            updated: Utc::now().date_naive(),
        }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn get_created(self) -> chrono::NaiveDate {
        self.created.clone()
    }

    fn get_updated(self) -> chrono::NaiveDate {
        self.updated.clone()
    }
}

pub async fn list_todo(db: &MySqlPool) -> Result<Vec<TodoDataBase>, sqlx::Error> {
    sqlx::query_as!(
        TodoDataBase,
        "SELECT id, title, content, created, updated FROM todo"
    )
    .fetch_all(db)
    .await
}

pub async fn fetch_todo_by_id(id: u32, db: &MySqlPool) -> Result<TodoDataBase, sqlx::Error> {
    sqlx::query_as!(
        TodoDataBase,
        "SELECT id, title, content, created, updated FROM todo WHERE id = ?",
        id
    )
    .fetch_one(db)
    .await
}

pub async fn count_todos(db: &MySqlPool) -> Result<usize, sqlx::Error> {
    let result = sqlx::query_as!(
        TodoDataBase,
        "SELECT id, title, content, created, updated FROM todo"
    )
    .fetch_all(db)
    .await;
    match result {
        Ok(rows) => Ok(rows.len()),
        Err(e) => Err(e),
    }
}
