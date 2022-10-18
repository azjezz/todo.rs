
use crate::task::database::repository;
use crate::http::HtmlTemplateResponse;

use actix_web::web;
use serde_json::json;

pub async fn route(repository: web::Data<repository::TaskRepository>) -> HtmlTemplateResponse {
    let tasks = repository.find_all();

    HtmlTemplateResponse::new(String::from("index.html"), json!({
        "tasks": tasks
    }))
}
