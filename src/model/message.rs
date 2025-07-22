use crate::schema::Message as MessageTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = MessageTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub author_id: Option<i32>,
    pub group_id: Option<i32>,
    pub content: String,
    pub time: std::time::SystemTime,
    pub proof: bool,
}

#[derive(Insertable)]
#[diesel(table_name = MessageTable)]
pub struct NewMessage<'a> {
    pub author_id: i32,
    pub group_id: i32,
    pub content: &'a str,
    pub time: std::time::SystemTime,
}
