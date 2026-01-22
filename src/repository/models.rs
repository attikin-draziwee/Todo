use ::chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::chrono};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TodoModel {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub created: chrono::NaiveDate,
    pub updated: chrono::NaiveDate,
}

impl TodoModel {
    pub fn new(id: u32, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            created: Utc::now().date_naive(),
            updated: Utc::now().date_naive(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoCreate {
    title: String,
    content: String,
}

impl TodoCreate {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

impl From<(String, String)> for TodoCreate {
    fn from(value: (String, String)) -> Self {
        Self {
            title: value.0,
            content: value.1,
        }
    }
}
