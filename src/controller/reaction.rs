use crate::model::reaction as reaction_model;
use crate::schema::Reaction;
use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_reaction(
    conn: &mut PgConnection,
    user_id: i32,
    message_id: i32,
    emoji: &str,
) -> Result<(), ControllerError> {
    let new_reaction = reaction_model::NewReaction {
        user_id,
        message_id,
        emoji,
    };
    diesel::insert_into(Reaction::table)
        .values(&new_reaction)
        .execute(conn)?;
    Ok(())
}

pub fn delete_reaction(
    conn: &mut PgConnection,
    id: i32,
    user_id: i32,
) -> Result<(), ControllerError> {
    let reaction: reaction_model::Reaction = Reaction::table.find(id).first(conn)?;
    if reaction.user_id != user_id {
        Err(ControllerError::AuthorizationError)
    } else {
        diesel::delete(Reaction::table.find(id)).execute(conn)?;
        Ok(())
    }
}
