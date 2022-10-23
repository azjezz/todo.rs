use crate::container::Container;

use actix_web::web::Data;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use tera::Tera;

pub trait CreatedFromContainer: Clone {
    fn create(container: &mut Container) -> Self;
}

impl<T: CreatedFromContainer + 'static> CreatedFromContainer for Data<T> {
    fn create(container: &mut Container) -> Data<T> {
        Data::new(container.get::<T>())
    }
}

impl CreatedFromContainer for Pool<ConnectionManager<PgConnection>> {
    fn create(container: &mut Container) -> Pool<ConnectionManager<PgConnection>> {
        r2d2::Pool::builder()
            .build(ConnectionManager::<PgConnection>::new(
                &container.settings.application.database_url,
            ))
            .map_err(|e| {
                println!(
                    "Failed to create postgreSQL pool ( {} ): {}",
                    container.settings.application.database_url, e
                );

                ::std::process::exit(1);
            })
            .unwrap()
    }
}

impl CreatedFromContainer for Tera {
    fn create(container: &mut Container) -> Tera {
        Tera::new(&container.settings.application.templates)
            .map_err(|e| {
                println!("Error(s) parsing tera templates: {}", e);

                ::std::process::exit(1);
            })
            .unwrap()
    }
}
