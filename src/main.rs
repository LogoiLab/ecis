#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::fs::FileServer;
use rocket_sync_db_pools::database;

use std::env;

pub mod category;
pub mod component;
pub mod location;
pub mod manufacturer;
pub mod schema;

#[database("postgres_db")]
struct DbConn(diesel::PgConnection);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/api", routes![index])
        .attach(DbConn::fairing())
}
