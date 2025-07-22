use crate::schema::UserWeeklyTarget as UserWeeklyTargetTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = UserWeeklyTargetTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserWeeklyTarget {
    pub id: i32,
    pub user_id: i32,
    pub groupchallenge_id: Option<i32>,
    pub target_count: i32,
    pub achieved_count: i32,
    pub current_progress: Option<String>,
    pub created_at: Option<std::time::SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = UserWeeklyTargetTable)]
pub struct NewUserWeeklyTarget {
    pub user_id: i32,
    pub groupchallenge_id: Option<i32>,
    pub target_count: i32,
}
