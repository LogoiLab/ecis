#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

pub mod category;
pub mod component;
pub mod location;
pub mod manufacturer;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("static/"))
        .mount("/api", routes![index])
}
