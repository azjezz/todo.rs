use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTask {
    pub content: String,
}
