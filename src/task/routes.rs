use crate::http::HtmlTemplateResponse;
use crate::http::RedirectResponse;
use crate::task::database::model;
use crate::task::database::repository;
use crate::task::input;

use actix_web::web;
use actix_web::HttpRequest;
use serde_json::json;

pub async fn index(request: HttpRequest) -> HtmlTemplateResponse {
    let tasks = request
        .app_data::<web::Data<repository::TaskRepository>>()
        .unwrap()
        .find_all();

    HtmlTemplateResponse::new("index.html", json!({ "tasks": tasks }))
}

pub async fn create(request: HttpRequest, input: web::Form<input::CreateTask>) -> RedirectResponse {
    request
        .app_data::<web::Data<repository::TaskRepository>>()
        .unwrap()
        .save(model::NewTask {
            content: &input.content,
            is_finished: &false,
        });

    RedirectResponse::to("/")
}

pub async fn finish(request: HttpRequest, id: web::Path<(i32,)>) -> RedirectResponse {
    request
        .app_data::<web::Data<repository::TaskRepository>>()
        .unwrap()
        .finish(id.into_inner().0);

    RedirectResponse::to("/")
}

pub async fn delete(request: HttpRequest, id: web::Path<(i32,)>) -> RedirectResponse {
    request
        .app_data::<web::Data<repository::TaskRepository>>()
        .unwrap()
        .delete(id.into_inner().0);

    RedirectResponse::to("/")
}
