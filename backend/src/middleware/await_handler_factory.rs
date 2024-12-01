use rocket::{Request, Data, tokio, Outcome};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::{Response, status};
use rocket::tokio::sync::Mutex;
use std::sync::Arc;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};

// A basic async middleware handler type
type MiddlewareHandler = Box<dyn Fn(&Request<'_>, &mut Data<'_>) -> rocket::async_trait + Send + Sync>;

pub fn await_handler_factory<F>(middleware: F) -> impl Fairing
where
    F: Fn(&Request<'_>, &mut Data<'_>) -> rocket::async_trait + Send + Sync + 'static,
{
    MiddlewareHandler {
        middleware: Box::new(middleware),
    }
}

pub struct MiddlewareHandler {
    middleware: MiddlewareHandler,
}

#[rocket::async_trait]
impl Fairing for MiddlewareHandler {
    fn info(&self) -> Info {
        Info {
            name: "Await Handler Factory Middleware",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, data: &mut Data<'_>) {
        let result = (self.middleware)(req, data).await;
        if let Err(e) = result {
            // Handle error, passing it to next middleware or handler
            req.local_cache(|| Response::new().set_status(Status::InternalServerError));
        }
    }
}
