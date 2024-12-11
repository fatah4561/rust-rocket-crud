use std::collections::BTreeMap;

use rocket::serde::{Deserialize, Serialize};

use crate::models::common_model::MapValueType;

#[derive(Serialize, Debug)]
pub struct GetAllQuestions {
    pub id: String,
    pub questions: String,
    pub validation: BTreeMap<String, MapValueType>,
    pub input_type: String,
    pub values: Vec<BTreeMap<String, MapValueType>>,
}

#[derive(Deserialize, Debug)]
pub struct PostAnswerRequest {
    pub form_id: String,
    pub answers: Vec<AnswerData>,
}

#[derive(Deserialize, Debug)]
pub struct AnswerData {
    pub id: String,
    pub question: String,
    pub values: ValueAnswer,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ValueAnswer {
    String(String),
    Array(Vec<ValueOption>),
}

#[derive(Deserialize, Debug)]
pub struct ValueOption {
    pub id: u32,
    pub text: String,
}
