use crate::schema::User as UserTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = UserTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub weekly_workout_goal: i32,
}

#[derive(Insertable)]
#[diesel(table_name = UserTable)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub salt: &'a str,
    pub weekly_workout_goal: i32,
}
