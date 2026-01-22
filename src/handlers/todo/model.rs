use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: usize,
    title: String,
    content: String,
}

impl Todo {
    pub fn new(id: usize, title: &str, content: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoCreate {
    title: String,
    content: String,
}

impl TodoCreate {
    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}
