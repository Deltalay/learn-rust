#[macro_use]
extern crate rocket;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use learn_rust::schema::url::{access_count, short_url};
use learn_rust::{already_exist, create_url, establish_connection, return_original_url};
use rocket::http::Status;
use rocket::response::{status, Redirect};
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

#[get("/<key>")]
async fn redirect_user<'a>(
    db: &State<DbConn>,
    key: &'a str,
) -> Result<Redirect, status::Custom<String>> {
    // TODO: LET TRY TO FIND THE KEY FIRST BEFORE SENDIMG ANY ERROR
    let mut conn = db.connection.lock().await;
    if already_exist(&mut *conn, key) {
        match return_original_url(&mut *conn, key) {
            Some(full_url) => {
                use learn_rust::schema::url::dsl::url;
                let update_access_view = diesel::update(url.filter(short_url.eq(key)))
                    .set(access_count.eq(access_count + 1))
                    .execute(&mut *conn)
                    .map_err(|err| err.to_string());
                match update_access_view {
                    Ok(_) => Ok(Redirect::to(full_url)),
                    Err(e) => Err(status::Custom(
                        Status::InternalServerError,
                        format!("Something went wrong: {}", e), 
                    )),
                }
            }
            None => Err(status::Custom(
                Status::NotFound,
                "The URL is not found in the database".to_string(),
            )),
        }
    } else {
        Err(status::Custom(
            Status::NotFound,
            "The URL is not found in the database".to_string(),
        ))
    }
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
        .mount("/", routes![index, create_url_route, redirect_user])
}
