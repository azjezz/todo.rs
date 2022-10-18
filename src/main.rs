mod database;
mod errors;
mod http;
mod task;

use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use serde_json::json;
use tera::Tera;

use crate::http::HtmlTemplateResponse;

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
            .route("/", web::get().to(task::routes::index))
            .route("/task", web::post().to(task::routes::create))
            .route("/task/finish/{id}", web::post().to(task::routes::finish))
            .route("/task/delete/{id}", web::post().to(task::routes::delete))
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(task_repository.clone()))
            .app_data(web::Data::new(tera.clone()))
            .default_service(web::to(|| async {
                HtmlTemplateResponse::new("errors/404.html", json!({}))
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
