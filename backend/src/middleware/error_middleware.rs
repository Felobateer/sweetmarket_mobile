use rocket::{Request, Response, tokio};
use rocket::http::Status;
use rocket::response::{Responder, content};
use rocket::serde::json::{Json, Value};
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

// Define a custom error structure
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: Status,
    pub message: String,
    pub data: Option<Value>,
}

impl ApiError {
    pub fn new(status: Status, message: String, data: Option<Value>) -> Self {
        ApiError { status, message, data }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'r> {
        // Log the error (simulating the log from JavaScript middleware)
        println!("[Error] {}", self.message);

        // If the status code is 500, change the message to "Internal server error"
        let message = if self.status == Status::InternalServerError || self.message.is_empty() {
            "Internal server error".to_string()
        } else {
            self.message
        };

        // Prepare the error response
        let error_response = json!({
            "response": false,
            "error": {
                "type": "error",
                "status": self.status.code,
                "message": message,
                "data": self.data,
            }
        });

        // Return the error response as JSON with the appropriate status code
        Response::build()
            .status(self.status)
            .sized_body(error_response.to_string().len(), std::io::Cursor::new(error_response.to_string()))
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

// Custom error catcher function
#[catch(500)]
fn internal_server_error() -> ApiError {
    ApiError::new(Status::InternalServerError, "Internal server error".to_string(), None)
}

// Global error handler (optional for catching all other errors)
#[catch(404)]
fn not_found() -> ApiError {
    ApiError::new(Status::NotFound, "Not Found".to_string(), None)
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .attach(rocket::fairing::AdHoc::on_ignite("Custom Error Middleware", |rocket| {
//             rocket.register("/", catchers![internal_server_error, not_found])
//         }))
// }
