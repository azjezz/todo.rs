use serde::Deserialize;

use crate::task::database::model::NewTask;

#[derive(Deserialize, Clone)]
pub struct CreateTask {
    pub content: String,
}

impl CreateTask {
    pub fn to_model(&self) -> NewTask {
        NewTask {
            content: &self.content,
            is_finished: &false,
        }
    }
}
