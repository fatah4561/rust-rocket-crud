
use crate::exception::error::{handle_error_response, CustomError};
use crate::models::common_model::Response;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> (Status, Json<Response<String>>) {
    handle_error_response(CustomError::not_found_error(format!(
        "No matching routes for {} {}",
        req.method(),
        req.uri()
    )))
}
