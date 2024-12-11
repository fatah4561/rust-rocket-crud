use crate::exception::error::handle_error_response;
use crate::helper::response::json_ok_response;
use crate::models::{
    common_model::Response, survey_model::GetAllQuestions, survey_model::PostAnswerRequest,
};
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
        rocket.manage(survey_controller).mount(
            "/survey",
            routes![get_questions_by_form_id, post_form_answer],
        )
    })
}

#[get("/<form_id>/questions")]
async fn get_questions_by_form_id(
    survey_controller: &State<SurveyController>,
    form_id: &str,
) -> (Status, Json<Response<Vec<GetAllQuestions>>>) {
    let res = survey_controller
        .survey_service
        .get_questions_by_form_id(form_id)
        .await;

    match res {
        Ok(res) => return json_ok_response(Some(res)),
        Err(e) => return handle_error_response(e),
    }
}

#[post("/<form_id>/answer", format = "json", data = "<answer_request>")]
async fn post_form_answer(
    survey_controller: &State<SurveyController>,
    form_id: &str,
    answer_request: Json<PostAnswerRequest>,
) -> (Status, Json<Response<()>>) {

    let req = answer_request.into_inner();
    let res = survey_controller
        .survey_service
        .post_form_answer(form_id, req)
        .await;

    match res {
        Ok(res) => return json_ok_response(Some(())),
        Err(e) => return handle_error_response(e),
    }
}
