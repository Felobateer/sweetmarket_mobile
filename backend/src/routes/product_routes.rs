use rocket::{get, post, put, delete, routes, serde::json::Json};
use std::sync::Arc;
use crate::models::{Products, Product};

#[get("/")]
pub fn get_all(products: &rocket::State<Arc<Products>>) -> Json<Vec<Product>> {
    products.get_all().map(Json).unwrap_or_else(|_| Json(vec![]))
}

#[get("/<id>")]
pub fn get_by_id(id: u32, products: &rocket::State<Arc<Products>>) -> Option<Json<Product>> {
    products.get_by_id(&id).ok().map(Json)
}

#[post("/", data = "<product>")]
pub fn add(product: Json<Product>, products: &rocket::State<Arc<Products>>) -> &'static str {
    if products.add(product.into_inner()).is_ok() {
        "Product added successfully"
    } else {
        "Failed to add product"
    }
}

#[put("/", data = "<product>")]
pub fn edit(product: Json<Product>, products: &rocket::State<Arc<Products>>) -> &'static str {
    if products.edit(product.into_inner()).is_ok() {
        "Product updated successfully"
    } else {
        "Failed to update product"
    }
}

#[delete("/<id>")]
pub fn delete(id: u32, products: &rocket::State<Arc<Products>>) -> &'static str {
    if products.delete(&id).is_ok() {
        "Product deleted successfully"
    } else {
        "Failed to delete product"
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_all, get_by_id, add, edit, delete]
}
