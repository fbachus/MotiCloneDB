use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL mus be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

use self::models::{Message, NewMessage};

pub fn create_message(
    conn: &mut PgConnection,
    author_id: i32,
    groups_id: i32,
    content: &str,
    proof: bool,
) -> Message {
    use crate::schema::messages;

    let new_message = NewMessage {
        author_id,
        groups_id,
        content,
        time: std::time::SystemTime::now(),
        proof,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(conn)
        .expect("Error saving new message")
}
