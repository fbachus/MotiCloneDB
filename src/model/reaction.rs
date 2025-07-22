use crate::schema::Reaction as ReactionTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = ReactionTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reaction {
    pub id: i32,
    pub user_id: i32,
    pub message_id: i32,
    pub emoji: String,
}

#[derive(Insertable)]
#[diesel(table_name = ReactionTable)]
pub struct NewReaction<'a> {
    pub user_id: i32,
    pub message_id: i32,
    pub emoji: &'a str,
}
