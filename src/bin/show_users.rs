use crate::models::*;
use diesel::prelude::*;
use moti_clone_db::*;

fn main() {
    use self::schema::users::dsl::*;
    println!("hello world");

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("{}", user.username);
        println!("{}", user.email);
    }
}
