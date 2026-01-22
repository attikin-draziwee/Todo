use serde::{Deserialize, Serialize};

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
