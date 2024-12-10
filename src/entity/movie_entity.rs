use rocket::serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use chrono::Utc;


#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub year: i32,
    pub plot: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub released: chrono::DateTime<Utc>,
}