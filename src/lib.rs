pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn generate_unique_string(length: i32, text: &str) -> &str {
    "Hellos"
}

pub fn create_url(conn: &mut SqliteConnection, short_url: &str, expire_date: Option<chrono::NaiveDateTime>) {
    use crate::schema::url;
    let shorten_url = "";
}
