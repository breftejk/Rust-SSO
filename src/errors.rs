use rocket::{Request, Response, response};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpError {
    error: bool,
    code: u16,
    message: String,
}

pub struct ErrorResponse(pub Json<HttpError>);

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let status = Status::from_code(self.0.code).unwrap_or(Status::InternalServerError);
        Response::build_from(self.0.respond_to(req)?)
            .status(status)
            .ok()
    }
}

pub fn http_error(code: u16, message: &str) -> ErrorResponse {
    ErrorResponse(Json(HttpError { error: true, code, message: String::from(message) }))
}