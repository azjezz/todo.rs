use crate::http::RedirectResponse;
use crate::task::data;
use crate::task::database::model;
use crate::task::database::repository;

use actix_web::web;

pub async fn route(
    repository: web::Data<repository::TaskRepository>,
    create_task: web::Form<data::CreateTask>,
) -> RedirectResponse {
    let new_task = model::NewTask {
        content: &create_task.content,
        is_finished: &false,
    };

    repository.save(new_task);

    RedirectResponse::to("/".to_string())
}
