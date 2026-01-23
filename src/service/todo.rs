use sqlx::{MySql, Pool};
use thiserror::Error;

use crate::repository::{
    self, conf,
    methods::insert_todo,
    models::{TodoCreate, TodoModel, TodoPatch, TodoPut},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool error: {0}")]
    MySQLError(#[from] sqlx::Error),
    #[error("Bad Request")]
    BadRequest,
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

    pub async fn delete_todo(&self, id: u32) -> Result<(), Error> {
        repository::methods::delete_todo(&self.pool, id).await?;
        Ok(())
    }

    pub async fn patch_todo(&self, id: u32, request: TodoPatch) -> Result<TodoModel, Error> {
        // Делаю проверку, т.к. если передать любое значение, то все равно будет статус 200 OK
        if request.content.is_none() && request.title.is_none() {
            return Err(Error::BadRequest);
        }
        let title: Option<String> = request.title;
        let content: Option<String> = request.content;
        Ok(repository::methods::patch_todo(&self.pool, id, title, content).await?)
    }

    pub async fn put_todo(&self, id: u32, request: TodoPut) -> Result<TodoModel, Error> {
        Ok(repository::methods::put_todo(&self.pool, id, request).await?)
    }
}
