
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub last_nickname: String,
    pub active: bool,
}