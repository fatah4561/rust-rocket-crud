use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: u8,
    pub status: String,
    pub data: Option<T>,
    pub errors: Option<String>,
}