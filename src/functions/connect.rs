use std::env;

use diesel::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

pub fn connect() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL set");
    SqliteConnection::establish(&db_url).expect("Cannot connect to the specified database")
}
