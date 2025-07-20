use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub weekly_workout_goal: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub author_id: i32,
    pub groups_id: i32,
    pub content: String,
    pub time: std::time::SystemTime,
    pub proof: bool,
}

use crate::schema::messages;
#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub author_id: i32,
    pub groups_id: i32,
    pub content: &'a str,
    pub time: std::time::SystemTime,
    pub proof: bool,
}
