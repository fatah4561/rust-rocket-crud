use crate::models::common_model::Response;

use rocket::http::Status;
use rocket::serde::json::Json;

pub fn json_ok_response<T>(data: Option<T>) -> (Status, Json<Response<T>>) {
    (
        Status::Ok,
        Json(Response {
            code: Status::Ok.code,
            status: Status::Ok.reason().unwrap().to_string(),
            data,
            errors: None,
        }),
    )
}
