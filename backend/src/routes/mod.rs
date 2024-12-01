use rocket::Route;
mod user_routes;

pub fn get_routes() -> Vec<Route> {
    routes![user_routes::index]
}
