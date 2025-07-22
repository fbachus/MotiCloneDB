use crate::model::userweeklytarget as userweeklytarget_model;
use crate::schema::UserWeeklyTarget;
use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_target(
    conn: &mut PgConnection,
    user_id: i32,
    groupchallenge_id: Option<i32>,
    target_count: i32,
) -> Result<(), ControllerError> {
    let new_user_weekly_target = userweeklytarget_model::NewUserWeeklyTarget {
        user_id,
        groupchallenge_id,
        target_count,
    };
    diesel::insert_into(UserWeeklyTarget::table)
        .values(&new_user_weekly_target)
        .execute(conn)?;
    Ok(())
}

pub fn change_target_count(
    conn: &mut PgConnection,
    target_id: i32,
    user_id: i32,
    new_target_count: i32,
) -> Result<(), ControllerError> {
    let challenge: userweeklytarget_model::UserWeeklyTarget =
        UserWeeklyTarget::table.find(target_id).first(conn)?;
    if user_id != challenge.user_id {
        Err(ControllerError::AuthorizationError)
    } else if new_target_count == challenge.target_count {
        Ok(())
    } else {
        diesel::update(UserWeeklyTarget::table.find(target_id))
            .set(UserWeeklyTarget::target_count.eq(new_target_count))
            .execute(conn)?;
        Ok(())
    }
}

fn change_achieved_count(
    conn: &mut PgConnection,
    target_id: i32,
    user_id: i32,
    new_achieved_count: i32,
) -> Result<(), ControllerError> {
    let challenge: userweeklytarget_model::UserWeeklyTarget =
        UserWeeklyTarget::table.find(target_id).first(conn)?;
    if user_id != challenge.user_id {
        Err(ControllerError::AuthorizationError)
    } else if new_achieved_count == challenge.achieved_count {
        Ok(())
    } else {
        diesel::update(UserWeeklyTarget::table.find(target_id))
            .set(UserWeeklyTarget::achieved_count.eq(new_achieved_count))
            .execute(conn)?;
        Ok(())
    }
}
