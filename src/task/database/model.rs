use crate::database::schema::tasks;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub is_finished: bool,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub content: &'a str,
    pub is_finished: &'a bool,
}
