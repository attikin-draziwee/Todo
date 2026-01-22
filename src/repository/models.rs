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
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_content(&self) -> &str {
        &self.content
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
