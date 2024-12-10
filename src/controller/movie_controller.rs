use crate::models;
use crate::service;
use crate::service::movie_service::MovieService;
use crate::service::movie_service::MovieServiceTrait;
use crate::exception::error::handle_error_response;

use std::sync::Arc;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub struct MovieController {
    movie_service: Arc<service::movie_service::MovieService>,
}

pub fn new_movie_controller(movie_service: Arc<MovieService>) -> MovieController {
    return MovieController { movie_service };
}

pub fn stage(movie_controller: MovieController) -> AdHoc {
    AdHoc::on_ignite("movie controller", |rocket| async {
        rocket
            .manage(movie_controller)
            .mount("/movies", routes![index, read])
    })
}

#[get("/")]
async fn index(
    movie_controller: &State<MovieController>,
) -> (
    Status,
    Json<models::common_model::Response<Vec<models::movie_model::GetAllMoviesResponse>>>,
) {
    let mut response = models::common_model::Response {
        code: Status::Ok.code,
        status: Status::Ok.reason().unwrap().to_string(),
        data: Some(vec![]),
        errors: None,
    };

    let movies = movie_controller.movie_service.get_all().await;

    response.data = match movies {
        Ok(m) => Some(m),
        Err(e) => {
            return handle_error_response(e)
        },
    };

    return (Status::Ok, Json(response));
}

#[get("/<id>")]
async fn read(
    movie_controller: &State<MovieController>,
    id: &str,
) -> (
    Status,
    Json<models::common_model::Response<models::movie_model::GetMovieDetailResponse>>,
) {
    let mut response = models::common_model::Response {
        code: Status::Ok.code,
        status: Status::Ok.reason().unwrap().to_string(),
        data: None,
        errors: None,
    };

    let movie = movie_controller
        .movie_service
        .get_detail(id.to_string())
        .await;

    match movie {
        Ok(movie) => {
            response.data = Some(movie);
            return (Status::Ok, Json(response));
        }
        Err(e) => {
            return handle_error_response(e);
        }
    }
}
