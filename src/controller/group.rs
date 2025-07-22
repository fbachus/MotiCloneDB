use crate::model::group as group_model;
use crate::schema::Group;
use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_group(conn: &mut PgConnection, user: i32, name: &str) -> Result<(), ControllerError> {
    let new_group = group_model::NewGroup {
        admin: user,
        name,
        challenge_deadline: None,
    };
    diesel::insert_into(Group::table)
        .values(&new_group)
        .execute(conn)?;
    Ok(())
}
fn check_user_is_group_admin(
    conn: &mut PgConnection,
    group_id: i32,
    user: i32,
) -> Result<(), ControllerError> {
    let tmp_group: group_model::Group = Group::table.find(group_id).first(conn)?;
    if user != tmp_group.admin {
        Err(ControllerError::AuthorizationError)
    } else {
        Ok(())
    }
}

pub fn delete_group(
    conn: &mut PgConnection,
    group_id: i32,
    user: i32,
) -> Result<(), ControllerError> {
    check_user_is_group_admin(conn, group_id, user)?;
    diesel::delete(Group::table.find(group_id)).execute(conn)?;
    Ok(())
}

pub fn change_admin(
    conn: &mut PgConnection,
    group_id: i32,
    user: i32,
    new_admin: i32,
) -> Result<(), ControllerError> {
    check_user_is_group_admin(conn, group_id, user)?;
    diesel::update(Group::table.find(group_id))
        .set(Group::admin.eq(new_admin))
        .execute(conn)?;
    Ok(())
}
