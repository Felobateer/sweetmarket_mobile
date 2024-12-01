use rocket::http::Status;
use rocket::serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct HttpException {
    pub status: u16,
    pub message: String,
    pub data: Option<serde_json::Value>, // Optional additional data
}

impl HttpException {
    pub fn new<S: Into<String>>(status: Status, message: S, data: Option<serde_json::Value>) -> Self {
        HttpException {
            status: status.code,
            message: message.into(),
            data,
        }
    }
}

impl std::fmt::Display for HttpException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {}: {}", self.status, self.message)
    }
}

impl Error for HttpException {}

impl<'r> rocket::response::Responder<'r, 'static> for HttpException {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let response_body = rocket::serde::json::Json(serde_json::json!({
            "status": self.status,
            "message": self.message,
            "data": self.data,
        }));

        rocket::Response::build()
            .status(Status::from_code(self.status).unwrap_or(Status::InternalServerError))
            .header(rocket::http::ContentType::JSON)
            .sized_body(response_body.to_string().len(), std::io::Cursor::new(response_body.to_string()))
            .ok()
    }
}
