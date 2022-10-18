use crate::task::database::model::NewTask;
use crate::task::database::model::Task;

use crate::database::pool::PostgresPool;

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

    pub fn finish(&self, identifier: i32) {
        use crate::database::schema::tasks::dsl::*;

        let connection = &mut self.get_connection();

        diesel::update(tasks.filter(id.eq(identifier)))
            .set(is_finished.eq(true))
            .execute(&mut *connection)
            .expect("failed to update task");
    }

    pub fn save(&self, model: NewTask) -> Task {
        use crate::database::schema::tasks::dsl::*;

        let connection = &mut self.get_connection();

        let data = diesel::insert_into(tasks)
            .values(&model)
            .get_result::<(i32, String, bool)>(connection)
            .expect("Error saving new account");

        Task {
            id: data.0,
            content: data.1,
            is_finished: data.2,
        }
    }

    pub fn delete(&self, identifier: i32) {
        use crate::database::schema::tasks::dsl::*;

        let connection = &mut self.get_connection();

        diesel::delete(tasks.filter(id.eq(identifier)))
            .execute(&mut *connection)
            .expect("failed to delete task");
    }

    pub fn find_all(&self) -> Vec<Task> {
        use crate::database::schema::tasks::dsl::*;

        let connection = &mut self.get_connection();

        tasks
            .order_by(id)
            .load::<Task>(&mut *connection)
            .expect("Error loading accounts")
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .get()
            .expect("couldn't get db connection from pool")
    }
}
