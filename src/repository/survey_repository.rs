use std::sync::Arc;

use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    Client,
};

use crate::entity::survey_entity;
use crate::exception::error::CustomError;

pub struct SurveyRepository {
    mongo: Arc<Client>,
}

pub fn new_survey_repository(mongo: Arc<Client>) -> SurveyRepository {
    SurveyRepository { mongo }
}

#[async_trait]
pub trait SurveyRepositoryTrait {
    async fn get_form_by_id(&self, id: ObjectId) -> Result<survey_entity::Form, CustomError>;
}

#[async_trait]
impl SurveyRepositoryTrait for SurveyRepository {
    async fn get_form_by_id(&self, id: ObjectId) -> Result<survey_entity::Form, CustomError> {
        let collection: mongodb::Collection<Document> =
            self.mongo.database("surveys").collection("forms");

        match collection.find_one(doc! {"_id": id}, None).await {
            Ok(Some(form)) => {
                let deserialized_doc: survey_entity::Form =
                    match bson::from_bson(Bson::Document(form)) {
                        Ok(movie) => movie,
                        Err(e) => {
                            return Err(CustomError::internal_server_error(e.to_string()));
                        }
                    };

                return Ok(deserialized_doc);
            }
            Ok(None) => {
                return Err(CustomError::not_found_error("Data not found".to_string()));
            }
            Err(e) => {
                return Err(CustomError::internal_server_error(e.to_string()));
            }
        }
    }
}
