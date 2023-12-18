use std::env;

use diesel::{sqlite::SqliteConnection, Connection};
use dotenvy::dotenv;

pub mod schema;
pub mod models;

pub fn connect() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL set");
    SqliteConnection::establish(&db_url).expect("Cannot connect to the specified database")
}

fn main() {
    println!("Hello, world!");
}
