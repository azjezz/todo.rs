use crate::http::response;
use crate::task;

use actix_web::web::ServiceConfig;
use actix_web::web::{get, post, to};

pub fn configure(config: &mut ServiceConfig) {
    config
        .route("/", get().to(task::routes::index))
        .route("/task", post().to(task::routes::create))
        .route("/task/finish/{id}", post().to(task::routes::finish))
        .route("/task/delete/{id}", post().to(task::routes::delete));

    config.default_service(to(|| async { response::template!("errors/404.html") }));
}
