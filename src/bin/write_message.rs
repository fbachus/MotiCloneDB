use moti_clone_db::*;
use std::io::{Read, stdin};

fn main() {
    let connection = &mut establish_connection();

    let author_id: i32 = 1;
    let groups_id: i32 = 1;
    let mut content = String::new();
    let proof: bool = false;

    println!("What will your message be? (Press {EOF} when finished)\n");
    stdin().read_to_string(&mut content).unwrap();

    let message = create_message(connection, author_id, groups_id, &content, proof);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
