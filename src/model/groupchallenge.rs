use crate::schema::GroupChallenge as GroupChallengeTable;
use chrono;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = GroupChallengeTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupChallenge {
    pub challenge_id: i32,
    pub group_id: i32,
    pub description: String,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = GroupChallengeTable)]
pub struct NewGroupChallenge<'a> {
    pub group_id: i32,
    pub description: &'a str,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
}
