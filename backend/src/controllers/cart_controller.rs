use std::sync::Arc;
use warp::Filter;

pub fn cart_routes(carts: Arc<Carts>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let carts = warp::any().map(move || carts.clone());

    let get_all = warp::path("carts")
        .and(warp::get())
        .and(carts.clone())
        .and_then(get_all_carts);

    let get_by_id = warp::path!("carts" / u32)
        .and(warp::get())
        .and(carts.clone())
        .and_then(get_cart_by_id);

    let create = warp::path("carts")
        .and(warp::post())
        .and(warp::body::json())
        .and(carts.clone())
        .and_then(create_cart);

    let update = warp::path!("carts" / u32)
        .and(warp::put())
        .and(warp::body::json())
        .and(carts.clone())
        .and_then(update_cart);

    let delete = warp::path!("carts" / u32)
        .and(warp::delete())
        .and(carts.clone())
        .and_then(delete_cart);

    get_all.or(get_by_id).or(create).or(update).or(delete)
}

async fn get_all_carts(carts: Arc<Carts>) -> Result<impl warp::Reply, warp::Rejection> {
    match carts.get_all() {
        Ok(all_carts) => Ok(warp::reply::json(&all_carts)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn get_cart_by_id(id: u32, carts: Arc<Carts>) -> Result<impl warp::Reply, warp::Rejection> {
    match carts.get_by_id(&id) {
        Ok(cart) => Ok(warp::reply::json(&cart)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn create_cart(new_cart: Cart, carts: Arc<Carts>) -> Result<impl warp::Reply, warp::Rejection> {
    match carts.add(new_cart) {
        Ok(_) => Ok(warp::reply::with_status("Cart created", warp::http::StatusCode::CREATED)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn update_cart(id: u32, updated_cart: Cart, carts: Arc<Carts>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut cart = updated_cart;
    cart.id = Some(id);
    match carts.edit(cart) {
        Ok(_) => Ok(warp::reply::with_status("Cart updated", warp::http::StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

async fn delete_cart(id: u32, carts: Arc<Carts>) -> Result<impl warp::Reply, warp::Rejection> {
    match carts.delete(&id) {
        Ok(_) => Ok(warp::reply::with_status("Cart deleted", warp::http::StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(ApiError(err))),
    }
}

#[derive(Debug)]
struct ApiError(String);

impl warp::reject::Reject for ApiError {}
