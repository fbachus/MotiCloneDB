use crate::schema::GroupMember as GroupMemberTable;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = GroupMemberTable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupMember {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
    pub joined_at: std::time::SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = GroupMemberTable)]
pub struct NewGroupMember {
    pub group_id: i32,
    pub user_id: i32,
    pub joined_at: std::time::SystemTime,
}
