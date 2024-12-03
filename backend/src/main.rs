#[macro_use]
extern crate rocket;

use std::sync::Arc;

mod routes;
mod models; // Assuming your models are in a module

use models::{Products, Carts, Payments, Rates};
use routes::{product_routes, user_routes, cart_routes, payment_routes, rating_routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Products::init().unwrap()))
        .manage(Arc::new(Carts::init().unwrap()))
        .manage(Arc::new(Payments::init().unwrap()))
        .manage(Arc::new(Rates::init().unwrap()))
        .mount("/user", user_routes::routes())
        .mount("/products", product_routes::routes())
        .mount("/carts", cart_routes::routes())
        .mount("/payments", payment_routes::routes())
        .mount("/rates", rating_routes::routes())
}
