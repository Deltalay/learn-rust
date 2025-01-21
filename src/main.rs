#[macro_use]
extern crate rocket;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use learn_rust::schema::url::{access_count, short_url};
use learn_rust::{already_exist, create_url, establish_connection, get_all, return_original_url};
use rocket::http::Status;
use rocket::response::{status, Redirect};
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::tokio::sync::Mutex;
use rocket::State;
use rocket_dyn_templates::{context, Template};

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
#[get("/all")]
async fn get_all_url(db: &State<DbConn>) -> Json<Vec<learn_rust::models::Url>> {
    let mut conn = db.connection.lock().await;
    let get_everything = get_all(&mut *conn);
    return Json(get_everything);
}
#[get("/<key>")]
async fn redirect_user<'a>(
    db: &State<DbConn>,
    key: &'a str,
) -> Result<Redirect, status::Custom<String>> {
    // TODO: Check expire
    let mut conn = db.connection.lock().await;
    if already_exist(&mut *conn, key) {
        match return_original_url(&mut *conn, key) {
            Some((full_url, Some(expiration))) => {
                let today = chrono::Local::now().naive_local();
                use learn_rust::schema::url::dsl::url;
                if expiration < today {
                    // Delete the URL because Why not?
                    diesel::delete(url.filter(short_url.eq(key)))
                        .execute(&mut *conn)
                        .expect("Error delete URL");
                    return Err(status::Custom(Status::Gone, "The URL has expired and soon be deleted from database. Please create new one.".to_string()));
                }
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
            Some((_, None)) => Err(status::Custom(
                Status::Gone,
                "The URL that you provide exist in our database, but it has expired.".to_string(),
            )),
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
// Too lazy to implement AUTH lol
#[get("/")]
fn index() -> Template {
    Template::render(
        "site/index",
        context! {
            title: "testing"
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DbConn {
            connection: establish_connection().into(),
        })
        .mount(
            "/",
            routes![index, create_url_route, redirect_user, get_all_url],
        )
        .attach(Template::fairing())
}
