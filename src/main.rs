mod app;
mod container;
mod database;
mod errors;
mod http;
mod macros;
mod task;

use crate::container::settings;
use crate::container::settings::Settings;
use crate::container::Container;
use crate::task::database::repository::TaskRepository;

use actix_settings::ApplySettings;
use actix_settings::Mode;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::initialize("actix.toml");

    configure_logger(&settings);

    HttpServer::new(enclose!(settings, {
        let mut container: Container = Container::new(&settings);

        App::new()
            .configure(app::configure)
            .app_data(container.get::<Data<Tera>>())
            .app_data(container.get::<Data<Pool<ConnectionManager<PgConnection>>>>())
            .app_data(container.get::<Data<TaskRepository>>())
    }))
    .apply_settings(&settings)
    .run()
    .await
}

fn configure_logger(settings: &Settings) {
    if settings.actix.enable_log {
        match settings.actix.mode {
            Mode::Development => {
                std::env::set_var("RUST_BACKTRACE", "1");
                std::env::set_var("RUST_LOG", "actix_web=debug");
            }
            Mode::Production => {
                std::env::set_var("RUST_LOG", "actix_web=info");
            }
        }

        env_logger::init();
    }
}
