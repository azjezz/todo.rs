use crate::http::response;
use crate::http::Responder;
use crate::task::database::repository::TaskRepository;
use crate::task::input::CreateTask;

use actix_web::web::{Data, Form, Path};

pub async fn index(repository: Data<TaskRepository>) -> Responder {
    match repository.find_all() {
        Ok(tasks) => response::template!("index.html", { "tasks": tasks }),
        Err(e) => response::error!(e),
    }
}

pub async fn create(repository: Data<TaskRepository>, input: Form<CreateTask>) -> Responder {
    match repository.save(input.to_model()) {
        Ok(_) => response::redirect_to!("/"),
        Err(e) => response::error!(e),
    }
}

pub async fn finish(repository: Data<TaskRepository>, id: Path<(i32,)>) -> Responder {
    match repository.find(id.into_inner().0) {
        Ok(Some(task)) => {
            if let Err(e) = repository.finish(task.id) {
                return response::error!(e);
            }

            response::redirect_to!("/")
        }
        Ok(None) => response::not_found!(),
        Err(e) => response::error!(e),
    }
}

pub async fn delete(repository: Data<TaskRepository>, id: Path<(i32,)>) -> Responder {
    match repository.find(id.into_inner().0) {
        Ok(Some(task)) => {
            if let Err(e) = repository.delete(task.id) {
                return response::error!(e);
            }

            response::redirect_to!("/")
        }
        Ok(None) => response::not_found!(),
        Err(e) => response::error!(e),
    }
}
