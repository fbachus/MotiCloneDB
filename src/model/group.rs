use crate::schema::Group as GroupTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = GroupTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: i32,
    pub referral_code: Option<String>,
    pub admin: i32,
    pub name: String,
    pub challenge_deadline: Option<std::time::SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = GroupTable)]
pub struct NewGroup<'a> {
    pub name: &'a str,
    pub admin: i32,
    pub challenge_deadline: Option<std::time::SystemTime>,
}
