use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use r2d2::Pool;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn create() -> Result<PostgresPool, r2d2::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
}
