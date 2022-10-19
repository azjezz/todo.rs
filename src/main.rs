mod database;
mod errors;
mod http;
mod task;

use actix_web::web::Data;
use actix_web::web::{get, post, to};
use actix_web::{App, HttpServer};
use serde_json::json;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let pool = match database::pool::create() {
            Ok(t) => t,
            Err(e) => {
                println!("Failed to create postgreSQL pool: {}", e);

                ::std::process::exit(1);
            }
        };

        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Error(s) parsing tera templates: {}", e);

                ::std::process::exit(1);
            }
        };

        let task_repository = task::database::repository::TaskRepository::new(pool.clone());

        App::new()
            .route("/", get().to(task::routes::index))
            .route("/task", post().to(task::routes::create))
            .route("/task/finish/{id}", post().to(task::routes::finish))
            .route("/task/delete/{id}", post().to(task::routes::delete))
            .app_data(Data::new(pool))
            .app_data(Data::new(task_repository))
            .app_data(Data::new(tera))
            .default_service(to(|| async {
                http::Responder::html_template("errors/404.html", json!({}))
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
