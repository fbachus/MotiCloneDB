use crate::controller::group::check_user_is_group_admin;
use crate::model::groupchallenge as groupchallenge_model;
use crate::schema::GroupChallenge;

use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_groupchallenge(
    conn: &mut PgConnection,
    user_id: i32,
    group_id: i32,
    description: &str,
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
) -> Result<(()), ControllerError> {
    check_user_is_group_admin(conn, group_id, user_id)?;
    let now = std::time::SystemTime::now();
    let new_group_challenge = groupchallenge_model::NewGroupChallenge {
        group_id,
        description,
        start_date,
        end_date,
        created_at: now,
        updated_at: now,
    };
    diesel::insert_into(GroupChallenge::table)
        .values(&new_group_challenge)
        .execute(conn)?;
    Ok(())
}

pub fn change_description(
    conn: &mut PgConnection,
    challenge_id: i32,
    user_id: i32,
    new_description: &str,
) -> Result<(), ControllerError> {
    let challenge: groupchallenge_model::GroupChallenge =
        GroupChallenge::table.find(challenge_id).first(conn)?;
    check_user_is_group_admin(conn, challenge.group_id, user_id)?;
    diesel::update(GroupChallenge::table.find(challenge_id))
        .set(GroupChallenge::description.eq(new_description))
        .execute(conn)?;
    Ok(())
}

pub fn change_dates(
    conn: &mut PgConnection,
    challenge_id: i32,
    user_id: i32,
    new_start_date: Option<chrono::NaiveDate>,
    new_end_date: Option<chrono::NaiveDate>,
) -> Result<(), ControllerError> {
    let challenge: groupchallenge_model::GroupChallenge =
        GroupChallenge::table.find(challenge_id).first(conn)?;
    check_user_is_group_admin(conn, challenge.group_id, user_id)?;
    match (new_start_date, new_end_date) {
        (None, None) => Ok(()),
        (Some(new_start_date), None) => {
            diesel::update(GroupChallenge::table.find(challenge_id))
                .set(GroupChallenge::start_date.eq(new_start_date))
                .execute(conn)?;
            Ok(())
        }
        (None, Some(new_end_date)) => {
            diesel::update(GroupChallenge::table.find(challenge_id))
                .set(GroupChallenge::end_date.eq(new_end_date))
                .execute(conn)?;
            Ok(())
        }
        (Some(new_start_date), Some(new_end_date)) => {
            diesel::update(GroupChallenge::table.find(challenge_id))
                .set((
                    GroupChallenge::start_date.eq(new_start_date),
                    GroupChallenge::end_date.eq(new_end_date),
                ))
                .execute(conn)?;
            Ok(())
        }
    }
}
