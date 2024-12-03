use std::sync::Arc;
use warp::Filter;

pub fn product_routes(products: Arc<Products>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let products = warp::any().map(move || products.clone());

    let get_all = warp::path("products")
        .and(warp::get())
        .and(products.clone())
        .and_then(get_all_products);

    let get_by_id = warp::path!("products" / u32)
        .and(warp::get())
        .and(products.clone())
        .and_then(get_product_by_id);

    let create = warp::path("products")
        .and(warp::post())
        .and(warp::body::json())
        .and(products.clone())
        .and_then(create_product);

    let update = warp::path!("products" / u32)
        .and(warp::put())
        .and(warp::body::json())
        .and(products.clone())
        .and_then(update_product);

    let delete = warp::path!("products" / u32)
        .and(warp::delete())
        .and(products.clone())
        .and_then(delete_product);

    get_all.or(get_by_id).or(create).or(update).or(delete)
}

async fn get_all_products(products: Arc<Products>) -> Result<impl warp::Reply, warp::Rejection> {
    match products.get_all() {
        Ok(all_products) => Ok(warp::reply::json(&all_products)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn get_product_by_id(id: u32, products: Arc<Products>) -> Result<impl warp::Reply, warp::Rejection> {
    match products.get_by_id(&id) {
        Ok(product) => Ok(warp::reply::json(&product)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn create_product(new_product: Product, products: Arc<Products>) -> Result<impl warp::Reply, warp::Rejection> {
    match products.add(new_product) {
        Ok(_) => Ok(warp::reply::with_status("Product created", warp::http::StatusCode::CREATED)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn update_product(id: u32, updated_product: Product, products: Arc<Products>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut product = updated_product;
    product.id = Some(id);
    match products.edit(product) {
        Ok(_) => Ok(warp::reply::with_status("Product updated", warp::http::StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn delete_product(id: u32, products: Arc<Products>) -> Result<impl warp::Reply, warp::Rejection> {
    match products.delete(&id) {
        Ok(_) => Ok(warp::reply::with_status("Product deleted", warp::http::StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

#[derive(Debug)]
struct ApiError(String);

impl warp::reject::Reject for ApiError {}
