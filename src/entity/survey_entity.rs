use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub category_id: u32,
    pub title: String,
    pub description: String,
    pub forms: Vec<FormQuestion>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormQuestion {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub question: String,
    pub answer_type: String,
    pub option: Option<Vec<String>>,
    pub required: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub minlength: Option<String>,
    pub maxlength: Option<String>,
}
