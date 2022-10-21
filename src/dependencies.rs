use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use std::env;
use tera::Tera;

pub fn get_database_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());

    match r2d2::Pool::builder().build(manager) {
        Ok(p) => p,
        Err(e) => {
            println!(
                "Failed to create postgreSQL pool ( {} ): {}",
                database_url, e
            );

            ::std::process::exit(1);
        }
    }
}

pub fn get_tera() -> Tera {
    match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Error(s) parsing tera templates: {}", e);

            ::std::process::exit(1);
        }
    }
}
