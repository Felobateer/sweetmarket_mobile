#[macro_use]
extern crate rocket;

mod config;
mod routes;

use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::config::<config::AppConfig>())
        .mount('/', routes::get_routes())
}