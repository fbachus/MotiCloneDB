use crate::controller::group::check_user_is_group_admin;
use crate::model::groupmember as groupmember_model;
use crate::schema::GroupMember;

use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_groupmember(
    conn: &mut PgConnection,
    group_id: i32,
    user_id: i32,
) -> Result<(), ControllerError> {
    let new_groupmember = groupmember_model::NewGroupMember {
        group_id,
        user_id,
        joined_at: std::time::SystemTime::now(),
    };
    diesel::insert_into(GroupMember::table)
        .values(&new_groupmember)
        .execute(conn)?;
    Ok(())
}

pub fn delete_groupmember(
    conn: &mut PgConnection,
    group_id: i32,
    user_requesting: i32,
    groupmember_target: i32,
) -> Result<(), ControllerError> {
    check_user_is_group_admin(conn, group_id, user_requesting)?;
    diesel::delete(GroupMember::table.find(groupmember_target)).execute(conn)?;
    Ok(())
}
