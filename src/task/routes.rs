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
    match repository.finish(id.into_inner().0) {
        Ok(affected_rows) => match affected_rows {
            // if no rows are effected, this means either:
            //    1. no task with the given id exists.
            //    2. the task with the given id exsits, but it is already finished.
            //
            // we could simply redirect back to the index route,
            // but i choose not to do that.
            0 => response::not_found!(),
            _ => response::redirect_to!("/"),
        },
        Err(e) => response::error!(e),
    }
}

pub async fn delete(repository: Data<TaskRepository>, id: Path<(i32,)>) -> Responder {
    let affected_rows = repository
        .delete(id.into_inner().0)
        .map_err(|e| response::error!(e))?;

    match affected_rows {
        // if no rows are effected, this means no task with the given id exists.
        // we could simply redirect back to the index route,
        // but i choose not to do that.
        0 => response::not_found!(),
        _ => response::redirect_to!("/"),
    }
}
