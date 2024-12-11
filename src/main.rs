#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;

mod config;
mod controller;
mod entity;
mod exception;
mod helper;
mod models;
mod repository;
mod service;

use rust_crud_mongo::not_found;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client = config::mongo::new().await;
    if let Err(ref e) = client {
        eprintln!("Error connecting to db: {e}")
    }
    let client = client.unwrap();

    // repository here
    let movie_repository: Arc<repository::movie_repository::MovieRepository> =
        repository::movie_repository::new_movie_repository(client.client.clone()).into();
    let survey_repository: Arc<repository::survey_repository::SurveyRepository> =
        repository::survey_repository::new_survey_repository(client.client.clone()).into();

    // service here
    let movie_service: Arc<service::movie_service::MovieService> =
        service::movie_service::new_movie_service(movie_repository.clone()).into();
    let survey_service: Arc<service::survey_service::SurveyService> =
        service::survey_service::new_survey_service(survey_repository.clone()).into();

    // controller here (controller used once so don't use Arc<>)
    let movie_controller =
        controller::movie_controller::new_movie_controller(movie_service.clone());
    let survey_controller =
        controller::survey_controller::new_survey_controller(survey_service.clone());

    rocket::build()
        .register("/", catchers![not_found])
        .attach(controller::movie_controller::stage(movie_controller))
        .attach(controller::survey_controller::stage(survey_controller))
}