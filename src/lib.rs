pub mod models;
pub mod schema;

use crate::schema::url::dsl::{short_url, url};
use base64::{prelude::BASE64_URL_SAFE, Engine};
use bcrypt::*;
use diesel::{dsl::exists, prelude::*, select};
use dotenvy::dotenv;
use models::{NewUrl, Url};
use rand::Rng;
use std::env;
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn generate_random_number(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(min..max);
    return n;
}
// TODO: Make sure it does not return any symbol.
// TURN OUT YOU CANNOT CHANGE THE FORMAT IN BCRYPT -_-
// SO I WILL JUST USE SHA256
pub fn generate_unique_string(length: u16, text: &str) -> String {
    if length > 60 {
        panic!("Length is out of bound!")
    }
    let base64_encode_url = BASE64_URL_SAFE.encode(text);
    let max_random_number_range = (base64_encode_url.len() + (length as usize)) as u32;
    let first_random_number = generate_random_number(0, max_random_number_range);
    let mut second_random_number = generate_random_number(0, max_random_number_range);
    while second_random_number == first_random_number {
        second_random_number = generate_random_number(0, max_random_number_range);
    }
    let first_random_position = generate_random_number(0, base64_encode_url.len() as u32) as usize;
    let mut second_random_position =
        generate_random_number(0, base64_encode_url.len() as u32) as usize;
    while second_random_position == first_random_position {
        second_random_position = generate_random_number(0, base64_encode_url.len() as u32) as usize;
    }
    let mut first_base64_clone = base64_encode_url.clone();
    let mut second_base64_clone = base64_encode_url.clone();
    first_base64_clone.insert(
        first_random_position,
        char::from_digit(first_random_number % 10, 10).unwrap_or('0'),
    );
    second_base64_clone.insert(
        second_random_position,
        char::from_digit(second_random_number % 10, 10).unwrap_or('0'),
    );
    let final_result = first_base64_clone + &second_base64_clone;
    let mut result = hash(final_result, DEFAULT_COST).unwrap();
    result = result[7..].to_string();
    result = sha256::digest(result);
    if result.len() > length as usize {
        result.truncate(length as usize);
    } else {
        while result.len() < length as usize {
            let random_number = generate_random_number(0, 10);
            let random_char = char::from_digit(random_number, 10).unwrap_or('0');
            result.push(random_char);
        }
    }
    result
}

pub fn already_exist(conn: &mut SqliteConnection, shorten_url: &str) -> bool {
    let check_exist_url = select(exists(url.filter(short_url.eq(shorten_url))))
        .get_result(conn)
        .expect("Error checking short URL existence");
    check_exist_url
}

pub fn create_url(
    conn: &mut SqliteConnection,
    long_url: &str,
    expire_date: Option<chrono::NaiveDateTime>,
) -> Url {
    use crate::schema::url;
    let mut shorten_url = generate_unique_string(7, long_url);
    while already_exist(conn, &shorten_url) {
        shorten_url = generate_unique_string(7, long_url);
    }
    let new_url = NewUrl {
        expires_at: expire_date,
        short_url: &shorten_url,
        long_url,
    };
    diesel::insert_into(url::table)
        .values(&new_url)
        .returning(Url::as_returning())
        .get_result(conn)
        .expect("Error saving new url")
        
}
