use rocket::{get, post, put, delete, routes, serde::json::Json};
use std::sync::Arc;
use crate::models::{Rates, Rate};

#[get("/")]
pub fn get_all(rates: &rocket::State<Arc<Rates>>) -> Json<Vec<Rate>> {
    rates.get_all().map(Json).unwrap_or_else(|_| Json(vec![]))
}

#[get("/<id>")]
pub fn get_by_id(id: u32, rates: &rocket::State<Arc<Rates>>) -> Option<Json<Rate>> {
    rates.get_by_id(&id).ok().map(Json)
}

#[post("/", data = "<rate>")]
pub fn add(rate: Json<Rate>, rates: &rocket::State<Arc<Rates>>) -> &'static str {
    if rates.add(rate.into_inner()).is_ok() {
        "Rate added successfully"
    } else {
        "Failed to add rate"
    }
}

#[put("/", data = "<rate>")]
pub fn edit(rate: Json<Rate>, rates: &rocket::State<Arc<Rates>>) -> &'static str {
    if rates.edit(rate.into_inner()).is_ok() {
        "Rate updated successfully"
    } else {
        "Failed to update rate"
    }
}

#[delete("/<id>")]
pub fn delete(id: u32, rates: &rocket::State<Arc<Rates>>) -> &'static str {
    if rates.delete(&id).is_ok() {
        "Rate deleted successfully"
    } else {
        "Failed to delete rate"
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all, get_by_id, add, edit, delete]
}
