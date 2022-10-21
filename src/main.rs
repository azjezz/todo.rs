mod database;
mod dependencies;
mod errors;
mod http;
mod task;

use crate::task::database::repository::TaskRepository;

use actix_web::web::Data;
use actix_web::web::{delete, get, post, patch, to};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        let pool = dependencies::get_database_pool();
        let tera = dependencies::get_tera();
        let task_repository = TaskRepository::new(pool.clone());

        App::new()
            .route("/", get().to(task::routes::index))
            .route("/task", post().to(task::routes::create))
            .route("/task/{id}", patch().to(task::routes::finish))
            .route("/task/{id}", delete().to(task::routes::delete))
            .default_service(to(|| async {
                http::response::template!("errors/404.html")
            }))
            .app_data(Data::new(pool))
            .app_data(Data::new(tera))
            .app_data(Data::new(task_repository))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
