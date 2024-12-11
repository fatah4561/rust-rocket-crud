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

// enums for HashMap / BTreeMap values type
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MapValueType {
    Boolean(bool),
    String(String),
    Unsigned(u64),
    Integer(i64),
    Float(f64),
}
