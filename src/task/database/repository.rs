use crate::database::pool::PostgresPool;
use crate::errors::Error;
use crate::task::database::model::NewTask;
use crate::task::database::model::Task;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;

#[derive(Clone)]
pub struct TaskRepository {
    pool: PostgresPool,
}

impl TaskRepository {
    pub fn new(pool: PostgresPool) -> Self {
        TaskRepository { pool }
    }

    pub fn find(&self, identifier: i32) -> Result<Option<Task>, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                let query = tasks.order_by(id).filter(id.eq(identifier));

                match query.first::<Task>(&mut *connection).optional() {
                    Ok(res) => Ok(res),
                    Err(e) => Err(Error::DatabaseQuery(e.to_string())),
                }
            }
            Err(error) => Err(error),
        }
    }

    pub fn find_all(&self) -> Result<Vec<Task>, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                let query = tasks.order_by(id);

                match query.load::<Task>(&mut *connection) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(Error::DatabaseQuery(e.to_string())),
                }
            }
            Err(error) => Err(error),
        }
    }

    pub fn save(&self, model: NewTask) -> Result<Task, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                let query = diesel::insert_into(tasks).values(&model);

                match query.get_result::<(i32, String, bool)>(&mut *connection) {
                    Ok(data) => Ok(Task {
                        id: data.0,
                        content: data.1,
                        is_finished: data.2,
                    }),
                    Err(e) => Err(Error::DatabaseQuery(e.to_string())),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn finish(&self, identifier: i32) -> Result<usize, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                let query =
                    diesel::update(tasks.filter(id.eq(identifier))).set(is_finished.eq(true));

                match query.execute(&mut *connection) {
                    Ok(results) => Ok(results),
                    Err(e) => Err(Error::DatabaseQuery(e.to_string())),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(&self, identifier: i32) -> Result<usize, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                let query = diesel::delete(tasks.filter(id.eq(identifier)));

                match query.execute(&mut *connection) {
                    Ok(r) => Ok(r),
                    Err(e) => Err(Error::DatabaseQuery(e.to_string())),
                }
            }
            Err(e) => Err(e),
        }
    }

    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        match self.pool.get() {
            Ok(c) => Ok(c),
            Err(e) => Err(Error::DatabaseConnection(e.to_string())),
        }
    }
}
