use crate::models::common_model::{ErrorResponse, Response};

use std::collections::HashMap;

use rocket::http::Status;
use rocket::serde::json::Json;

pub enum ErrorType {
    InternalServerError, // db error, parsing, etc
    BadRequest,
    Validation,
    NotFound,
}

pub struct CustomError {
    pub error_type: ErrorType,
    pub message: String,
    pub field: Option<String>
}

impl CustomError {
    pub fn internal_server_error(message: String) -> CustomError {
        CustomError{
            error_type: ErrorType::InternalServerError,
            message,
            field: None,
        }
    }

    pub fn bad_request_error(message: String, field: Option<String>) -> CustomError {
        CustomError {
            error_type: ErrorType::BadRequest,
            message,
            field,
        }
    }
    
    pub fn validation_error(message: String, field: Option<String>) -> CustomError {
        CustomError {
            error_type: ErrorType::Validation,
            message,
            field,
        }
    }

    pub fn not_found_error(message: String) -> CustomError{
        CustomError {
            error_type: ErrorType::NotFound,
            message,
            field: None,
        }
    }

}

pub fn handle_error_response<T>(err: CustomError) ->  (Status, Json<Response<T>>) {
    let mut status_code = Status::InternalServerError;
    let mut status = Status::InternalServerError.reason().unwrap_or_default().to_string();
    let mut code = Status::InternalServerError.code;
    let errors ;

    match err.error_type {
        ErrorType::BadRequest | ErrorType::Validation => {
            status_code = Status::BadRequest;
            code = Status::BadRequest.code;
            status = Status::BadRequest.reason().unwrap_or_default().to_string();
        },
        ErrorType::NotFound => {
            status_code = Status::NotFound;
            code = Status::NotFound.code;
            status = Status::NotFound.reason().unwrap_or_default().to_string();
        },
        _ => (), // stay internal server error
    }

    match err.field {
        Some(field) => {
            let mut error_map = HashMap::new();
            error_map.insert(field, err.message);
            errors = Some(ErrorResponse::Object(error_map));
        },
        None => {
            errors = Some(ErrorResponse::String(err.message))
        },
    }
    (status_code, Json(Response { code, status, data: None, errors }))
}