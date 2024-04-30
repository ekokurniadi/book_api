#[macro_use]
extern crate rocket;

pub mod connection;
pub mod controller;
pub mod models;
pub mod repository;
pub mod routes;
pub mod schema;
pub mod shared;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![controller::books::get_all])
}
