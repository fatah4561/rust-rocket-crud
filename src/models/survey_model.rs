use std::collections::BTreeMap;

use bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

use crate::entity::survey_entity::QuestionAnswer;
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
    Number(i64),
    String(String),
    Array(Vec<ValueOption>),
}

#[derive(Deserialize, Debug)]
pub struct ValueOption {
    pub id: u32,
    pub text: String,
}

impl From<AnswerData> for QuestionAnswer {
    fn from(answer: AnswerData) -> Self {
        QuestionAnswer {
            id: Some(ObjectId::parse_str(answer.id).unwrap_or_default()),
            question: answer.question,
            values: Some(answer.values.into()),
        }
    }
}