use rocket::serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: u16,
    pub status: String,
    pub data: Option<T>,
    pub errors: Option<ErrorResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorResponse {
    String(String),
    Object(HashMap<String, String>),
}
