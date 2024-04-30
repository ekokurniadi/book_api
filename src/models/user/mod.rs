use diesel::prelude::*;
use  crate::schema::users;
use rocket::serde::Serialize;


#[derive(Queryable, Selectable, Debug, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}