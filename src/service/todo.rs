use sqlx::{MySql, Pool};
use thiserror::Error;

use crate::repository::{
    self, conf,
    methods::insert_todo,
    models::{TodoCreate, TodoModel},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool error: {0}")]
    MySQLError(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct Service {
    pool: Pool<MySql>,
}

impl Service {
    pub async fn new(db_url: &str) -> Result<Self, Error> {
        Ok(Self {
            pool: conf::configure_db(db_url).await?,
        })
    }

    pub async fn create_todo(&self, request: TodoCreate) -> Result<TodoModel, Error> {
        let todo = insert_todo(&self.pool, request).await?;
        Ok(todo)
    }

    pub async fn get_todo(&self, id: u32) -> Option<TodoModel> {
        repository::methods::fetch_todo_by_id(id, &self.pool)
            .await
            .map_err(|err| {
                eprintln!("Fetch todo by id failed: {err}");
            })
            .ok()
    }

    pub async fn get_list_todo(&self) -> Option<Vec<TodoModel>> {
        repository::methods::list_todo(&self.pool)
            .await
            .map_err(|err| {
                eprintln!("Fetch todo list failed: {err}");
            })
            .ok()
    }
}
