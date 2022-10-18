mod database;
mod http;
mod task;

use actix_web::{web, App, HttpServer};
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
            .route("/", web::get().to(task::routes::index::route))
            .route("/task/create", web::post().to(task::routes::create::route))
            .route("/task/delete/{id}", web::post().to(task::routes::delete::route))
            .route("/task/finish/{id}", web::post().to(task::routes::finish::route))

            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(task_repository.clone()))
            .app_data(web::Data::new(tera.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
