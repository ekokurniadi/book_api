use crate::schema::book;
use diesel::prelude::*;
use rocket::serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize, Ord, Eq, PartialEq, PartialOrd)]
#[diesel(table_name = book)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Books {
    pub id: i32,
    pub name: String,
}
