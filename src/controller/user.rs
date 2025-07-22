use crate::model::user as user_model;
use crate::schema::User;
use argon_hash_password;
use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_user(
    conn: &mut PgConnection,
    username: &str,
    email: &str,
    password: &str,
    weekly_workout_goal: i32,
) -> Result<(), ControllerError> {
    let (hash, salt) =
        argon_hash_password::create_hash_and_salt(password).expect("Failed Password hashing");
    let new_user = user_model::NewUser {
        username,
        email,
        password: &hash,
        salt: &salt,
        weekly_workout_goal,
    };
    diesel::insert_into(User::table)
        .values(&new_user)
        .returning(User::id)
        .execute(conn)?;
    Ok(())
}

fn validate_user(
    conn: &mut PgConnection,
    user_id: i32,
    password: &str,
) -> Result<user_model::User, ControllerError> {
    let tmp_user: user_model::User = User::table.find(user_id).first(conn)?;

    let passwords_match = argon_hash_password::check_password_matches_hash(
        password,
        &tmp_user.password,
        &tmp_user.salt,
    )
    .expect("password hashing failed");
    if passwords_match {
        Ok(tmp_user)
    } else {
        Err(ControllerError::WrongPasswordError)
    }
}

pub fn delete_user(
    conn: &mut PgConnection,
    user_id: i32,
    password: &str,
) -> Result<(), ControllerError> {
    let valid_user = validate_user(conn, user_id, password)?;
    diesel::delete(User::table.find(valid_user.id)).execute(conn)?;
    Ok(())
}

pub fn change_email(
    conn: &mut PgConnection,
    user_id: i32,
    new_email: &str,
    password: &str,
) -> Result<(), ControllerError> {
    let valid_user = validate_user(conn, user_id, password)?;

    diesel::update(User::table.find(valid_user.id))
        .set(User::email.eq(new_email))
        .execute(conn)?;
    Ok(())
}

pub fn change_username(
    conn: &mut PgConnection,
    user_id: i32,
    current_username: &str,
    new_username: &str,
) -> Result<(), ControllerError> {
    if current_username != new_username {
        let tmp_user: user_model::User = User::table.find(user_id).first(conn)?;
        diesel::update(User::table.find(tmp_user.id))
            .set(User::username.eq(new_username))
            .execute(conn)?;
    }
    Ok(())
}

pub fn change_weekly_workout_goal(
    conn: &mut PgConnection,
    user_id: i32,
    current_goal: i32,
    new_goal: i32,
) -> Result<(), ControllerError> {
    if current_goal != new_goal {
        let tmp_user: user_model::User = User::table.find(user_id).first(conn)?;
        diesel::update(User::table.find(tmp_user.id))
            .set(User::weekly_workout_goal.eq(new_goal))
            .execute(conn)?;
    }
    Ok(())
}
