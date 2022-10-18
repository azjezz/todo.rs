use crate::errors::Error;
use crate::http::HtmlTemplateResponse;
use crate::http::RedirectResponse;
use crate::task::database::model;
use crate::task::database::repository;
use crate::task::input;

use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde_json::json;

pub async fn index(request: HttpRequest) -> HttpResponse {
    if let Some(repository) = request.app_data::<web::Data<repository::TaskRepository>>() {
        return match repository.find_all() {
            Ok(tasks) => HtmlTemplateResponse::new("index.html", json!({ "tasks": tasks }))
                .respond_to(&request),
            Err(error) => HtmlTemplateResponse::error(error).respond_to(&request),
        };
    }

    HtmlTemplateResponse::error(Error::MissingDependencyError("TaskRepository"))
        .respond_to(&request)
}

pub async fn create(request: HttpRequest, input: web::Form<input::CreateTask>) -> HttpResponse {
    if let Some(repository) = request.app_data::<web::Data<repository::TaskRepository>>() {
        let model = model::NewTask {
            content: &input.content,
            is_finished: &false,
        };

        return match repository.save(model) {
            Ok(_) => RedirectResponse::to("/").respond_to(&request),
            Err(error) => HtmlTemplateResponse::error(error).respond_to(&request),
        };
    }

    HtmlTemplateResponse::error(Error::MissingDependencyError("TaskRepository"))
        .respond_to(&request)
}

pub async fn finish(request: HttpRequest, id: web::Path<(i32,)>) -> HttpResponse {
    if let Some(repository) = request.app_data::<web::Data<repository::TaskRepository>>() {
        return match repository.finish(id.into_inner().0) {
            Ok(_) => RedirectResponse::to("/").respond_to(&request),
            Err(error) => HtmlTemplateResponse::error(error).respond_to(&request),
        };
    }

    HtmlTemplateResponse::error(Error::MissingDependencyError("TaskRepository"))
        .respond_to(&request)
}

pub async fn delete(request: HttpRequest, id: web::Path<(i32,)>) -> HttpResponse {
    if let Some(repository) = request.app_data::<web::Data<repository::TaskRepository>>() {
        return match repository.delete(id.into_inner().0) {
            Ok(_) => RedirectResponse::to("/").respond_to(&request),
            Err(error) => HtmlTemplateResponse::error(error).respond_to(&request),
        };
    }

    HtmlTemplateResponse::error(Error::MissingDependencyError("TaskRepository"))
        .respond_to(&request)
}
