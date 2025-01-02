use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::url)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Url {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub access_count: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::url)]
pub struct NewUrl<'a> {
  pub short_url: &'a str,
  pub long_url: &'a str,
  pub expires_at: Option<chrono::NaiveDateTime>
}


