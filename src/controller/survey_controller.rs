use crate::exception::error::handle_error_response;
use crate::helper::response::json_ok_response;
use crate::models;
use crate::service::survey_service::{SurveyService, SurveyServiceTrait};

use std::sync::Arc;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub struct SurveyController {
    survey_service: Arc<SurveyService>,
}

pub fn new_survey_controller(survey_service: Arc<SurveyService>) -> SurveyController {
    SurveyController { survey_service }
}

pub fn stage(survey_controller: SurveyController) -> AdHoc {
    AdHoc::on_ignite("survey controller", |rocket| async {
        rocket
            .manage(survey_controller)
            .mount("/survey", routes![get_questions_by_form_id])
    })
}

#[get("/<form_id>/questions")]
async fn get_questions_by_form_id(
    survey_controller: &State<SurveyController>,
    form_id: &str,
) -> (
    Status,
    Json<models::common_model::Response<Vec<models::survey_model::GetAllQuestions>>>,
) {
    let res = survey_controller
        .survey_service
        .get_questions_by_form_id(form_id.to_string())
        .await;

    match res {
        Ok(res) => return json_ok_response(Some(res)),
        Err(e) => return handle_error_response(e),
    }
}
