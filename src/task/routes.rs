use crate::errors::Error;
use crate::http::Responder;
use crate::task::database::model::NewTask;
use crate::task::database::repository::TaskRepository;
use crate::task::input::CreateTask;

use actix_web::web::{Data, Form, Path};
use serde_json::json;

pub async fn index(repository: Data<TaskRepository>) -> Responder {
    match repository.find_all() {
        Ok(tasks) => Responder::html_template("index.html", json!({ "tasks": tasks })),
        Err(e) => Responder::error(e),
    }
}

pub async fn create(repository: Data<TaskRepository>, input: Form<CreateTask>) -> Responder {
    let model = NewTask {
        content: &input.content,
        is_finished: &false,
    };

    match repository.save(model) {
        Ok(_) => Responder::redirect_to("/"),
        Err(e) => Responder::error(e),
    }
}

pub async fn finish(repository: Data<TaskRepository>, id: Path<(i32,)>) -> Responder {
    match repository.find(id.into_inner().0) {
        Ok(task) => match task {
            Some(t) => match repository.finish(t.id) {
                Ok(_) => Responder::redirect_to("/"),
                Err(e) => Responder::error(e),
            },
            None => Responder::error(Error::NotFound("Task not found.".to_owned())),
        },
        Err(e) => Responder::error(e),
    }
}

pub async fn delete(repository: Data<TaskRepository>, id: Path<(i32,)>) -> Responder {
    match repository.find(id.into_inner().0) {
        Ok(task) => match task {
            Some(t) => match repository.delete(t.id) {
                Ok(_) => Responder::redirect_to("/"),
                Err(e) => Responder::error(e),
            },
            None => Responder::error(Error::NotFound("Task not found.".to_owned())),
        },
        Err(e) => Responder::error(e),
    }
}
