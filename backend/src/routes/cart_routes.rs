use rocket::{get, post, put, delete, routes, serde::json::Json};
use std::sync::Arc;
use crate::models::{Carts, Cart};

#[get("/")]
pub fn get_all(carts: &rocket::State<Arc<Carts>>) -> Json<Vec<Cart>> {
    carts.get_all().map(Json).unwrap_or_else(|_| Json(vec![]))
}

#[get("/<id>")]
pub fn get_by_id(id: u32, carts: &rocket::State<Arc<Carts>>) -> Option<Json<Cart>> {
    carts.get_by_id(&id).ok().map(Json)
}

#[post("/", data = "<cart>")]
pub fn add(cart: Json<Cart>, carts: &rocket::State<Arc<Carts>>) -> &'static str {
    if carts.add(cart.into_inner()).is_ok() {
        "Cart added successfully"
    } else {
        "Failed to add cart"
    }
}

#[put("/", data = "<cart>")]
pub fn edit(cart: Json<Cart>, carts: &rocket::State<Arc<Carts>>) -> &'static str {
    if carts.edit(cart.into_inner()).is_ok() {
        "Cart updated successfully"
    } else {
        "Failed to update cart"
    }
}

#[delete("/<id>")]
pub fn delete(id: u32, carts: &rocket::State<Arc<Carts>>) -> &'static str {
    if carts.delete(&id).is_ok() {
        "Cart deleted successfully"
    } else {
        "Failed to delete cart"
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all, get_by_id, add, edit, delete]
}
