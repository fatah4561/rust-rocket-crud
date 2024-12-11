use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use rocket::serde::{Deserialize, Serialize};

use crate::models::survey_model;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct FormAnswer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "form_id", skip_serializing_if = "Option::is_none")]
    pub form_id: Option<ObjectId>,
    pub agent_id: String,
    pub answer: Vec<QuestionAnswer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionAnswer {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub question: String,
    pub values: Option<ValueAnswer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ValueAnswer {
    Number(i64),
    String(String),
    Array(Vec<ValueOption>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueOption {
    pub id: u32,
    pub text: String,
}

impl From<survey_model::ValueAnswer> for ValueAnswer {
    fn from(value: survey_model::ValueAnswer) -> Self {
        match value {
            survey_model::ValueAnswer::Number(n) => ValueAnswer::Number(n),
            survey_model::ValueAnswer::String(s) => ValueAnswer::String(s),
            survey_model::ValueAnswer::Array(a) => {
                ValueAnswer::Array(a.into_iter().map(From::from).collect())
            }
        }
    }
}

impl From<survey_model::ValueOption> for ValueOption {
    fn from(option: survey_model::ValueOption) -> Self {
        ValueOption {
            id: option.id,
            text: option.text,
        }
    }
}
