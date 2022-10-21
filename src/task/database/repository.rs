use crate::errors::Error;
use crate::task::database::model::NewTask;
use crate::task::database::model::Task;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use r2d2::Pool;

#[derive(Clone)]
pub struct TaskRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl TaskRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        TaskRepository { pool }
    }

    // pub fn find(&self, identifier: i32) -> Result<Option<Task>, Error> {
    //     match self.get_connection() {
    //         Ok(mut connection) => {
    //             use crate::database::schema::tasks::dsl::*;
    //
    //             tasks
    //                 .order_by(id)
    //                 .filter(id.eq(identifier))
    //                 .first::<Task>(&mut *connection)
    //                 .optional()
    //                 .map_err(Error::from)
    //         }
    //         Err(error) => Err(error),
    //     }
    // }

    pub fn find_all(&self) -> Result<Vec<Task>, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                tasks
                    .order_by(id)
                    .load::<Task>(&mut *connection)
                    .map_err(Error::from)
            }
            Err(error) => Err(error),
        }
    }

    pub fn save(&self, model: NewTask) -> Result<Task, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                diesel::insert_into(tasks)
                    .values(&model)
                    .get_result::<(i32, String, bool)>(&mut *connection)
                    .map(|data| Task {
                        id: data.0,
                        content: data.1,
                        is_finished: data.2,
                    })
                    .map_err(Error::from)
            }
            Err(e) => Err(e),
        }
    }

    pub fn finish(&self, identifier: i32) -> Result<usize, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                diesel::update(
                    tasks
                        .filter(id.eq(identifier))
                        .filter(is_finished.eq(false)),
                )
                .set(is_finished.eq(true))
                .execute(&mut *connection)
                .map_err(Error::from)
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(&self, identifier: i32) -> Result<usize, Error> {
        match self.get_connection() {
            Ok(mut connection) => {
                use crate::database::schema::tasks::dsl::*;

                diesel::delete(tasks.filter(id.eq(identifier)))
                    .execute(&mut *connection)
                    .map_err(Error::from)
            }
            Err(e) => Err(e),
        }
    }

    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        self.pool.get().map_err(Error::from)
    }
}
