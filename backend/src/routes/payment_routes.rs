use rocket::{get, post, put, delete, routes, serde::json::Json};
use std::sync::Arc;
use crate::models::{Payments, Payment};

#[get("/")]
pub fn get_all(payments: &rocket::State<Arc<Payments>>) -> Json<Vec<Payment>> {
    payments.get_all().map(Json).unwrap_or_else(|_| Json(vec![]))
}

#[get("/<id>")]
pub fn get_by_id(id: u32, payments: &rocket::State<Arc<Payments>>) -> Option<Json<Payment>> {
    payments.get_by_id(&id).ok().map(Json)
}

#[post("/", data = "<payment>")]
pub fn add(payment: Json<Payment>, payments: &rocket::State<Arc<Payments>>) -> &'static str {
    if payments.add(payment.into_inner()).is_ok() {
        "Payment added successfully"
    } else {
        "Failed to add payment"
    }
}

#[put("/", data = "<payment>")]
pub fn edit(payment: Json<Payment>, payments: &rocket::State<Arc<Payments>>) -> &'static str {
    if payments.edit(payment.into_inner()).is_ok() {
        "Payment updated successfully"
    } else {
        "Failed to update payment"
    }
}

#[delete("/<id>")]
pub fn delete(id: u32, payments: &rocket::State<Arc<Payments>>) -> &'static str {
    if payments.delete(&id).is_ok() {
        "Payment deleted successfully"
    } else {
        "Failed to delete payment"
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all, get_by_id, add, edit, delete]
}
