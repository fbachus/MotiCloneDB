use crate::model::message as message_model;
use crate::schema::Message;
use diesel::prelude::*;
use moti_clone_db::ControllerError;

pub fn create_message(
    conn: &mut PgConnection,
    user_id: i32,
    group_id: i32,
    content: &str,
) -> Result<(), ControllerError> {
    let new_message = message_model::NewMessage {
        author_id: user_id,
        group_id,
        content,
        time: std::time::SystemTime::now(),
    };
    diesel::insert_into(Message::table)
        .values(&new_message)
        .execute(conn)?;
    Ok(())
}

//TODO: editing and deleting messages, optimally with "last_edited" field
