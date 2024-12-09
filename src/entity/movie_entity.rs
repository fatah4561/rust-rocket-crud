use rocket::serde::{Deserialize, Serialize};
use mongodb::bson::{doc, oid::ObjectId};
use chrono::Utc;


#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    year: i32,
    plot: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    released: chrono::DateTime<Utc>,
}