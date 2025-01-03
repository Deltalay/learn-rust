#[macro_use]
extern crate rocket;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use learn_rust::{create_url, establish_connection};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio::sync::Mutex;
use rocket::State;

struct DbConn {
    connection: Mutex<SqliteConnection>,
}

//TODO: Implement backend first
// FINISH CREATE
// TODO: Implement Retrieve, and Delete
#[derive(Deserialize, Debug, PartialEq)]
#[serde(crate = "rocket::serde")]
struct URL<'r> {
    url: &'r str,
    expire: Option<chrono::NaiveDateTime>,
}

#[post("/create_url", data = "<url_data>")]
async fn create_url_route(
    url_data: Json<URL<'_>>,
    db: &State<DbConn>,
) -> Json<learn_rust::models::Url> {
    let mut conn = db.connection.lock().await;
    let expire = match url_data.expire {
        Some(expire_time) => expire_time,
        None => Utc::now().naive_utc() + Duration::days(1),
    };
    let data: learn_rust::models::Url = create_url(&mut *conn, url_data.url, Some(expire));
    return Json(data);
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DbConn {
            connection: establish_connection().into(),
        })
        .mount("/", routes![index, create_url_route])
}
