use crate::entity::survey_entity::{self, FormAnswer};
use crate::exception::error::CustomError;
use crate::helper::validation::insert_unsigned_validation;
use crate::models::{common_model::MapValueType, survey_model};
use crate::repository::survey_repository::{SurveyRepository, SurveyRepositoryTrait};

use std::collections::BTreeMap;
use std::sync::Arc;

use mongodb::bson::oid::ObjectId;

pub struct SurveyService {
    survey_repository: Arc<SurveyRepository>,
}

pub fn new_survey_service(survey_repository: Arc<SurveyRepository>) -> SurveyService {
    SurveyService { survey_repository }
}

#[async_trait]
pub trait SurveyServiceTrait {
    async fn get_questions_by_form_id(
        &self,
        id: &str,
    ) -> Result<Vec<survey_model::GetAllQuestions>, CustomError>;
    async fn post_form_answer(
        &self,
        id: &str,
        answers: survey_model::PostAnswerRequest
    ) -> Result<(), CustomError>;
}

#[async_trait]
impl SurveyServiceTrait for SurveyService {
    async fn get_questions_by_form_id(
        &self,
        id: &str,
    ) -> Result<Vec<survey_model::GetAllQuestions>, CustomError> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => {
                return Err(CustomError::bad_request_error(
                    "format id invalid".to_string(),
                    Some("id".to_string()),
                ));
            }
        };

        let res = self.survey_repository.get_form_by_id(obj_id).await?;

        let mut questions = vec![];
        for data in res.forms {
            let mut data_id = "".to_string();
            if data.id.is_some() {
                data_id = data.id.unwrap().to_hex()
            };

            let mut validations: BTreeMap<String, MapValueType> = BTreeMap::new();
            // validation
            {
                if let Some(required) = data.required {
                    if required.to_lowercase() == "required".to_string() {
                        validations.insert("required".to_string(), MapValueType::Boolean(true));
                    }
                }

                insert_unsigned_validation(&mut validations, "min", data.min.as_deref())?;
                insert_unsigned_validation(&mut validations, "max", data.max.as_deref())?;
                insert_unsigned_validation(
                    &mut validations,
                    "maxlength",
                    data.maxlength.as_deref(),
                )?;
                insert_unsigned_validation(
                    &mut validations,
                    "minlength",
                    data.minlength.as_deref(),
                )?;
            } // end validation

            let mut values = vec![];
            if let Some(option) = data.option {
                for (i, v) in option.iter().enumerate() {
                    let mut option_map = BTreeMap::new();
                    option_map.insert("id".to_string(), MapValueType::Unsigned(i as u64));
                    option_map.insert("value".to_string(), MapValueType::String(v.to_string()));
                    values.push(option_map);
                }
            }

            let question = survey_model::GetAllQuestions {
                id: data_id,
                questions: data.question,
                validation: validations,
                input_type: data.answer_type,
                values,
            };

            questions.push(question);
        }
        return Ok(questions);
    }

    async fn post_form_answer(
        &self,
        id: &str,
        answers: survey_model::PostAnswerRequest
    ) -> Result<(), CustomError> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(id) => id,
            Err(_) => {
                return Err(CustomError::bad_request_error(
                    "format id invalid".to_string(),
                    Some("id".to_string()),
                ));
            }
        };

        let mut new_form_answer = FormAnswer {
            id: None,
            form_id: Some(obj_id),
            agent_id: "".to_string(), // TODO! replace with real
            answer: vec![],
        };

        for answer in answers.answers {
            let question_answer_id = match ObjectId::parse_str(answer.id) {
                Ok(id) => id,
                Err(_) => {
                    return Err(CustomError::bad_request_error(
                        "format id invalid".to_string(),
                        Some("id".to_string()),
                    ));
                }
            };
            
            let question_answer = survey_entity::QuestionAnswer{ 
                id: Some(question_answer_id), 
                question: answer.question, 
                values: Some(answer.values.into()) 
            };
            new_form_answer.answer.push(question_answer);
        }

        self.survey_repository.insert_answer(new_form_answer).await?;

        Ok(())
    }
}
