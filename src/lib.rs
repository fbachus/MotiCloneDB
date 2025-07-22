use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use thiserror::Error;

pub mod model;
pub mod schema;

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("could not delete database entry")]
    DeleteEntryError(#[from] diesel::result::Error),
    #[error("wrong password provided")]
    WrongPasswordError,
    #[error("no permission")]
    AuthorizationError,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
